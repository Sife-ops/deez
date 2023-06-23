use anyhow::Result;
use aws_sdk_dynamodb::types::{DeleteRequest, PutRequest, WriteRequest};
use deez::*;
use std::collections::HashMap;

use super::super::schemas::foo::{init, Task, TaskItems};
use super::super::schemas::make_client;

#[tokio::test]
async fn batch_write() -> Result<()> {
    init().await;
    let c = make_client().await;

    batch_write!(
        c;
        writes:
            Task{
                task_id: "aaa".to_string(),
                project: "bbb".to_string(),
                employee: "ccc".to_string(),
                ..Default::default()
            },
            Task{
                task_id: "aaa".to_string(),
                project: "fff".to_string(),
                employee: "ggg".to_string(),
                ..Default::default()
            },
            Task{
                task_id: "aaa".to_string(),
                project: "hhh".to_string(),
                employee: "iii".to_string(),
                ..Default::default()
            };
        deletes:
    )?;

    {
        let keys = Task {
            task_id: "aaa".to_string(),
            ..Default::default()
        }
        .primary_keys();

        let v = vec_from_query!(
            c
                .query()
                .table_name(Task::table_name())
                .key_condition_expression("#pk = :pk")
                .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), keys.hash.field())])))
                .set_expression_attribute_values(Some(HashMap::from([(":pk".to_string(), keys.hash.av())])))
                .send()
                .await?

            => TaskItems
        );

        // println!("{:#?}", v);
        assert_eq!(v.len(), 3);
    }

    batch_write!(
        c;
        writes: ;
        deletes:
            Task{
                task_id: "aaa".to_string(),
                project: "bbb".to_string(),
                employee: "ccc".to_string(),
                ..Default::default()
            },
            Task{
                task_id: "aaa".to_string(),
                project: "hhh".to_string(),
                employee: "iii".to_string(),
                ..Default::default()
            }
    )?;

    {
        let keys = Task {
            task_id: "aaa".to_string(),
            ..Default::default()
        }
        .primary_keys();

        let v = vec_from_query!(
            c
                .query()
                .table_name(Task::table_name())
                .key_condition_expression("#pk = :pk")
                .set_expression_attribute_names(Some(HashMap::from([("#pk".to_string(), keys.hash.field())])))
                .set_expression_attribute_values(Some(HashMap::from([(":pk".to_string(), keys.hash.av())])))
                .send()
                .await?

            => TaskItems
        );

        // println!("{:#?}", v);
        assert_eq!(v.len(), 1);
    }

    Ok(())
}
