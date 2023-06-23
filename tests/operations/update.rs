use anyhow::Result;
use aws_sdk_dynamodb::types::AttributeValue;
use deez::*;
use std::collections::HashMap;

use super::super::schemas::foo::{init, Task, TaskItems};
use super::super::schemas::make_client;

#[tokio::test]
async fn update() -> Result<()> {
    init().await;

    let c = make_client().await;

    create!(c; Task {
        task_id: "aaa".to_string(),
        project: "bbb".to_string(),
        employee: "ccc".to_string(),
        description: "ddd".to_string(),
        ..Default::default()
    })?;

    let k = Task {
        task_id: "aaa".to_string(),
        project: "bbb".to_string(),
        employee: "ccc".to_string(),
        ..Default::default()
    }
    .primary_keys();
    // .index_keys_av(Index::Primary);

    let u: HashMap<String, AttributeValue> = Task {
        description: "lol".to_string(),
        ..Default::default()
    }
    .into();

    c.update_item()
        .table_name(Task::table_name())
        .set_key(Some(HashMap::from([
            (k.hash.field.clone(), k.hash.av()),
            (k.range.field.clone(), k.range.av()),
        ])))
        .update_expression("SET #u = :u")
        .set_expression_attribute_names(Some(HashMap::from([(
            "#u".to_string(),
            "description".to_string(),
        )])))
        .set_expression_attribute_values(Some(HashMap::from([(
            ":u".to_string(),
            u["description"].clone(),
        )])))
        .send()
        .await?;

    let r = vec_from_query!(
        c
            .query()
            .table_name(Task::table_name())
            .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), k.hash.field.clone()),
                ("#sk".to_string(), k.range.field.clone()),
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                (":pk".to_string(), k.hash.av()),
                (":sk".to_string(), k.range.av()),
            ])))
            .send()
            .await?

        => TaskItems
    );

    // println!("{:#?}", r);

    let f = r.first().unwrap();

    assert_eq!(f.description, "lol".to_string());

    Ok(())
}
