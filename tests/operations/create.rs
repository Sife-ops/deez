use crate::schemas::foo::{init, Foo, Task, TaskItems};
use crate::schemas::make_client;
use anyhow::{Ok, Result};
use deez::*;
use std::collections::HashMap;
use std::sync::Arc;

#[ignore]
#[tokio::test]
async fn temp() -> Result<()> {
    init().await;
    let client = make_client().await;

    create!(client; Foo {
        string_1: Some("foo".to_string()),
        ..Default::default()
    })?;

    // let b: HashMap<String, AttributeValue> = a.into();
    // println!("{:#?}", b);

    Ok(())
}

#[tokio::test]
async fn create() -> Result<()> {
    init().await;
    let client = make_client().await;

    // task
    create!(client; Task {
        task_id: Some("123".to_string()),
        project: Some("foo_proj".to_string()),
        employee: Some("foo_empl".to_string()),
        description: "foo_desc".to_string(),
        some_metadata: "abcd".to_string(),
    })?;

    let task_keys = Task {
        task_id: Some("123".to_string()),
        project: Some("foo_proj".to_string()),
        employee: Some("foo_empl".to_string()),
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

    {
        let first = tasks.first().unwrap();
        assert_eq!(first.task_id, Some("123".to_string()));
        assert_eq!(first.project, Some("foo_proj".to_string()));
        assert_eq!(first.employee, Some("foo_empl".to_string()));
        assert_eq!(first.description, "foo_desc");
        assert_eq!(first.some_metadata, "it's true");
    }

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
            create!(cc; Task {
                task_id: Some("foo".to_string()),
                project: Some(i.to_string()),
                employee: Some(i.to_string()),
                description:i.to_string(),
                some_metadata:i.to_string(),
            })
            .unwrap();
        }));
    }

    for i in v {
        i.await?;
    }

    let f = Task {
        task_id: Some("foo".to_string()),
        ..Default::default()
    }
    .primary_keys();

    let i = vec_from_query!(
        c
            .query()
            .table_name(Task::table_name())
            .key_condition_expression("#pk = :pk")
            .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), f.hash.field())])))
            .set_expression_attribute_values(Some(HashMap::from([(":pk".to_string(), f.hash.av())])))
            .send()
            .await?

        => TaskItems
    );

    // println!("{}", i.len());

    assert_eq!(i.len(), 10);

    Ok(())
}
