use anyhow::{Ok, Result};
use deez::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::schemas::foo::TaskItems;

use super::super::schemas::foo::{init, Foo, FooItems, Task};
use super::super::schemas::make_client;

#[tokio::test]
async fn create() -> Result<()> {
    init().await;
    let client = make_client().await;

    create!(client; Task {
        task_id: "123".to_string(),
        project: "foo_proj".to_string(),
        employee: "foo_empl".to_string(),
        description: "foo_desc".to_string(),
        some_metadata: "abcd".to_string(),
    })?;

    let task_keys = Task {
        task_id: "123".to_string(),
        project: "foo_proj".to_string(),
        employee: "foo_empl".to_string(),
        ..Default::default()
    }
    .primary_keys();

    let tasks = vec_from_query!(
        client
            .query()
            .table_name(Task::table_name())
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
            .await?

        => TaskItems
    );

    // println!("{:#?}", tasks);

    let first = tasks.first().unwrap();
    assert_eq!(first.task_id, "123");
    assert_eq!(first.project, "foo_proj");
    assert_eq!(first.employee, "foo_empl");
    assert_eq!(first.description, "foo_desc");
    assert_eq!(first.some_metadata, "");

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
            create!(cc; Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: i.to_string(),
                foo_string_3: i.to_string(),
                foo_string_4: i.to_string(),
                foo_string_5: i.to_string(),
                foo_string_6: i.to_string(),
                ..Default::default()
            })
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

    let i = vec_from_query!(
        c
            .query()
            .table_name(Foo::table_name())
            .key_condition_expression("#pk = :pk")
            .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), f.hash.field())])))
            .set_expression_attribute_values(Some(HashMap::from([(":pk".to_string(), f.hash.av())])))
            .send()
            .await?

        => FooItems
    );

    // println!("{}", i.len());

    assert_eq!(i.len(), 10);

    Ok(())
}
