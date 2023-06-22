use anyhow::{Ok, Result};
use deez::*;
use std::collections::HashMap;
use std::sync::Arc;

use super::super::schemas::foo::{init, Foo, FooItems};
use super::super::schemas::make_client;

macro_rules! create {
    ($client:expr; $inst:expr) => {{
        let inst = $inst;

        $client
            .put_item()
            .table_name(inst.table__name())
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                (
                    "#pk".to_string(),
                    inst.index_key(Index::Primary, Key::Hash).field,
                ),
                (
                    "#sk".to_string(),
                    inst.index_key(Index::Primary, Key::Range).field,
                ),
            ])))
            .set_item(Some(inst.into()))
            .send()
            .await?
    }};
}

macro_rules! query {
    (
        $client:expr;
        $table_name:expr;
        $key_cond_expr:expr;
        $($x:expr => $y:expr),+;
        $($a:expr => $b:expr),+
    ) => {{
        $client
            .query()
            .table_name($table_name)
            .key_condition_expression($key_cond_expr)
            .set_expression_attribute_names(Some(HashMap::from([
                $(( $x.to_string(), $y )),+
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                $(( $a.to_string(), $b )),+
            ])))
    }};

    (
        $client:expr;
        $table_name:expr;
        $key_cond_expr:expr;
        $($x:expr => $y:expr),+;
        $($a:expr => $b:expr),+
        => $items:ident
    ) => {{
        let q = query!(
            $client;
            $table_name;
            $key_cond_expr;
            $( $x => $y ),+;
            $( $a => $b ),+
        )
        .send()
        .await?;

        $items::from(q.items().unwrap()).items()
    }};

    (
        $client:expr;
        $table_name:expr;
        $index_name:expr;
        $key_cond_expr:expr;
        $($x:expr => $y:expr),+;
        $($a:expr => $b:expr),+
        => $items:ident
    ) => {{
        let q = query!(
            $client;
            $table_name;
            $key_cond_expr;
            $( $x => $y ),+;
            $( $a => $b ),+
        )
        .index_name($index_name)
        .send()
        .await?;

        $items::from(q.items().unwrap()).items()
    }};
}

// let asdf = create!(client; f.clone());
// let fdsa = query!(client; f.clone() => FooItems);
// f.index_key_av(Index::Primary, Key::Hash).

#[tokio::test]
async fn query_macro() -> Result<()> {
    init().await;
    let client = make_client().await;

    // todo: nothing to read

    let keys = Foo {
        foo_string_1: "aaa".to_string(),
        foo_string_2: "bbb".to_string(),
        foo_string_3: "ccc".to_string(),
        foo_string_4: "ddd".to_string(),
        foo_string_5: "eee".to_string(),
        foo_string_6: "fff".to_string(),
        ..Default::default()
    }
    .index_keys_av(Index::Primary);

    let items = query!(
        client;
        Foo::table_name();
        "#pk = :pk and begins_with(#sk, :sk)";
        "#pk" => keys.hash.field,
        "#sk" => keys.range.field;
        ":pk" => keys.hash.composite,
        ":sk" => keys.range.composite
        => FooItems
    );

    Ok(())
}

#[tokio::test]
async fn create() -> Result<()> {
    init().await;
    let client = make_client().await;

    let f = Foo {
        foo_string_1: "aaa".to_string(),
        foo_string_2: "bbb".to_string(),
        foo_string_3: "ccc".to_string(),
        foo_string_4: "ddd".to_string(),
        foo_string_5: "eeee".to_string(),
        foo_string_6: "fff".to_string(),
        ..Default::default()
    };

    client
        .put_item()
        .table_name(Foo::table_name())
        .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            (
                "#pk".to_string(),
                f.index_key(Index::Primary, Key::Hash).field,
            ),
            (
                "#sk".to_string(),
                f.index_key(Index::Primary, Key::Range).field,
            ),
        ])))
        .set_item(Some(f.clone().into()))
        .send()
        .await?;

    let ff = f.index_keys_av(Index::Primary);
    let q = client
        .query()
        .table_name(Foo::table_name())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), ff.hash.field),
            ("#sk".to_string(), ff.range.field),
        ])))
        .set_expression_attribute_values(Some(HashMap::from([
            (":pk".to_string(), ff.hash.composite),
            (":sk".to_string(), ff.range.composite),
        ])))
        .send()
        .await?;

    let i = FooItems::from(q.items().unwrap()).items();
    let ii = i.first().unwrap();

    // println!("{:#?}", ii);

    assert_eq!(ii.foo_string_1, "aaa");
    assert_eq!(ii.foo_string_2, "bbb");
    assert_eq!(ii.foo_string_3, "ccc");
    assert_eq!(ii.foo_string_4, "ddd");
    assert_eq!(ii.foo_string_5, "eeee");
    assert_eq!(ii.foo_string_6, "");
    assert_eq!(ii.foo_num1, 69.0);
    assert_eq!(ii.foo_bool1, true);

    Ok(())
}

#[tokio::test]
async fn create_with_threads() -> Result<()> {
    init().await;
    let c = Arc::new(make_client().await);

    let mut v = Vec::with_capacity(10);
    for i in 0..10 {
        let cc = Arc::clone(&c);

        v.push(tokio::spawn(async move {
            let f = Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: i.to_string(),
                foo_string_3: i.to_string(),
                foo_string_4: i.to_string(),
                foo_string_5: i.to_string(),
                foo_string_6: i.to_string(),
                ..Default::default()
            };

            cc.put_item()
                .table_name(Foo::table_name())
                .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
                .set_expression_attribute_names(Some(HashMap::from([
                    (
                        "#pk".to_string(),
                        f.index_key(Index::Primary, Key::Hash).field,
                    ),
                    (
                        "#sk".to_string(),
                        f.index_key(Index::Primary, Key::Range).field,
                    ),
                ])))
                .set_item(Some(f.clone().into()))
                .send()
                .await
                .unwrap();
        }));
    }

    for i in v {
        i.await?;
    }

    let f = Foo {
        foo_string_1: "foo".to_string(),
        ..Default::default()
    };
    let ff = f.index_keys_av(Index::Primary);

    let q = c
        .query()
        .table_name(Foo::table_name())
        .key_condition_expression("#pk = :pk")
        .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), ff.hash.field)])))
        .set_expression_attribute_values(Some(HashMap::from([(
            ":pk".to_string(),
            ff.hash.composite,
        )])))
        .send()
        .await?;

    let i = FooItems::from(q.items().unwrap()).items();

    // println!("{}", i.len());

    assert_eq!(i.len(), 10);

    Ok(())
}
