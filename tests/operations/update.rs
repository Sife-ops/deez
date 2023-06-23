use aws_sdk_dynamodb::types::AttributeValue;
use deez::*;
use std::collections::HashMap;

use super::super::schemas::foo::{init, Task, TaskItems};
use super::super::schemas::make_client;

#[tokio::test]
async fn update() {
    init().await;

    let c = make_client().await;

    let t = Task {
        task_id: "aaa".to_string(),
        project: "bbb".to_string(),
        employee: "ccc".to_string(),
        description: "ddd".to_string(),
        ..Default::default()
    };

    c.put_item()
        .table_name(Task::table_name())
        .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), t.primary_key(Key::Hash).field),
            ("#sk".to_string(), t.primary_key(Key::Range).field),
        ])))
        .set_item(Some(t.into()))
        .send()
        .await
        .unwrap();

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
        .await
        .unwrap();

    let q = c
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
        .await
        .unwrap();

    let r = TaskItems::from(q.items().unwrap()).items();
    // println!("{:#?}", r);

    let f = r.first().unwrap();

    assert_eq!(f.description, "lol".to_string());
}
