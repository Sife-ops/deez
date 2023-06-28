use crate::schemas::{
    foo::{init, Task, TaskItems},
    make_client,
};
use anyhow::Result;
use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};
use deez::*;
use std::collections::HashMap;

#[tokio::test]
async fn delete() -> Result<()> {
    init().await;
    let c = make_client().await;

    batch_write!(
        c;
        writes:
            Task {
                task_id: Some("aaa".to_string()),
                project: Some("bbb".to_string()),
                employee: Some("ccc".to_string()),
                ..Default::default()
            },
            Task {
                task_id: Some("aaa".to_string()),
                project: Some("hhh".to_string()),
                employee: Some("iii".to_string()),
                ..Default::default()
            };
        deletes:
    )?;

    ////////////////////////////////////////////////////////////////////////////

    remove!(c; Task {
        task_id: Some("aaa".to_string()),
        project: Some("hhh".to_string()),
        employee: Some("iii".to_string()),
        ..Default::default()
    })?;

    ////////////////////////////////////////////////////////////////////////////

    let keys = Task {
        task_id: Some("aaa".to_string()),
        ..Default::default()
    }
    .primary_keys();

    let q = vec_from_query!(
        c
            .query()
            .table_name(Task::table_name())
            .key_condition_expression("#pk = :pk")
            .set_expression_attribute_names(Some(HashMap::from([(
                "#pk".to_string(),
                keys.hash.field(),
            )])))
            .set_expression_attribute_values(Some(HashMap::from([(
                ":pk".to_string(),
                keys.hash.av(),
            )])))
            .send()
            .await?

        => TaskItems
    );

    // println!("{:#?}", q);
    assert_eq!(q.len(), 1);

    Ok(())
}
