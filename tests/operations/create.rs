use anyhow::{Ok, Result};
use deez::*;
use std::collections::HashMap;
use std::sync::Arc;

use super::super::schemas::foo::{init, Foo, FooItems, Task};
use super::super::schemas::make_client;

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
            ("#pk".to_string(), f.primary_key(Key::Hash).field()),
            ("#sk".to_string(), f.primary_key(Key::Range).field()),
        ])))
        .set_item(Some(f.clone().into()))
        .send()
        .await?;

    let ff = f.primary_keys();
    let q = client
        .query()
        .table_name(Foo::table_name())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), ff.hash.field()),
            ("#sk".to_string(), ff.range.field()),
        ])))
        .set_expression_attribute_values(Some(HashMap::from([
            (":pk".to_string(), ff.hash.av()),
            (":sk".to_string(), ff.range.av()),
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
async fn create_macro() -> Result<()> {
    init().await;
    let client = make_client().await;

    create!(client; Task {
        task_id: "123".to_string(),
        project: "foo_proj".to_string(),
        employee: "foo_empl".to_string(),
        description: "foo_desc".to_string(),
        some_metadata: "abcd".to_string(),
    });

    let task_keys = Task {
        task_id: "123".to_string(),
        project: "foo_proj".to_string(),
        employee: "foo_empl".to_string(),
        ..Default::default()
    }
    .primary_keys();

    let q = client
        .query()
        .table_name(Foo::table_name())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), task_keys.hash.field()),
            ("#sk".to_string(), task_keys.range.field()),
        ])))
        .set_expression_attribute_values(Some(HashMap::from([
            (":pk".to_string(), task_keys.hash.av()),
            (":sk".to_string(), task_keys.range.av()),
        ])))
        .send()
        .await?;

    Ok(())
}

#[ignore]
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
                    ("#pk".to_string(), f.primary_key(Key::Hash).field()),
                    ("#sk".to_string(), f.primary_key(Key::Range).field()),
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
    }
    .primary_keys();

    let q = c
        .query()
        .table_name(Foo::table_name())
        .key_condition_expression("#pk = :pk")
        .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), f.hash.field())])))
        .set_expression_attribute_values(Some(HashMap::from([(":pk".to_string(), f.hash.av())])))
        .send()
        .await?;

    let i = FooItems::from(q.items().unwrap()).items();

    // println!("{}", i.len());

    assert_eq!(i.len(), 10);

    Ok(())
}
