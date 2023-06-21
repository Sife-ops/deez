use deez::*;
use std::collections::HashMap;
use std::sync::Arc;

use super::super::schemas::foo::{init, Foo, FooItems};
use super::super::schemas::make_client;

#[tokio::test]
#[ignore]
async fn create() {
    init().await;

    let c = make_client().await;

    let f = Foo {
        foo_string_1: "aaa".to_string(),
        foo_string_2: "bbb".to_string(),
        foo_string_3: "ccc".to_string(),
        foo_string_4: "ddd".to_string(),
        foo_string_5: "eee".to_string(),
        foo_string_6: "fff".to_string(),
        ..Default::default()
    };

    c.put_item()
        .table_name(f.table())
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

    let ff = f.index_keys_av(Index::Primary);
    let q = c
        .query()
        .table_name(f.table())
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
        .await
        .unwrap();

    let i = FooItems::from(q.items().unwrap()).items();
    let ii = i.first().unwrap();

    println!("{:#?}", ii);

    assert_eq!(ii.foo_string_1, "aaa");
    assert_eq!(ii.foo_string_2, "bbb");
    assert_eq!(ii.foo_string_3, "ccc");
    assert_eq!(ii.foo_string_4, "ddd");
    assert_eq!(ii.foo_string_5, "eee");
    assert_eq!(ii.foo_string_6, "");
    assert_eq!(ii.foo_num1, 69.0);
    assert_eq!(ii.foo_bool1, true);
}

#[tokio::test]
async fn create_with_threads() {
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
                .table_name(f.table())
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
        i.await.unwrap();
    }

    let f = Foo {
        foo_string_1: "foo".to_string(),
        ..Default::default()
    };
    let ff = f.index_keys_av(Index::Primary);

    let q = c
        .query()
        .table_name(f.table())
        .key_condition_expression("#pk = :pk")
        .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), ff.hash.field)])))
        .set_expression_attribute_values(Some(HashMap::from([(
            ":pk".to_string(),
            ff.hash.composite,
        )])))
        .send()
        .await
        .unwrap();

    let i = FooItems::from(q.items().unwrap()).items();

    // println!("{}", i.len());

    assert_eq!(i.len(), 10);
}
