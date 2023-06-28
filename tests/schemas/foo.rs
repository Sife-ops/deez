use super::make_client;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
    ProjectionType, ScalarAttributeType,
};
use deez::*;
use std::collections::HashMap;

pub async fn init() {
    let client = make_client().await;

    let table_name = "foo_table";

    if let Ok(_) = client.delete_table().table_name(table_name).send().await {}

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .key_type(KeyType::Hash)
                .attribute_name("pk")
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .key_type(KeyType::Range)
                .attribute_name("sk")
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("foo_gsi1")
                .key_schema(
                    KeySchemaElement::builder()
                        .key_type(KeyType::Hash)
                        .attribute_name("gsi1pk")
                        .build(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .key_type(KeyType::Range)
                        .attribute_name("gsi1sk")
                        .build(),
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi1pk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi1sk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("foo_gsi2")
                .key_schema(
                    KeySchemaElement::builder()
                        .key_type(KeyType::Hash)
                        .attribute_name("gsi2pk")
                        .build(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .key_type(KeyType::Range)
                        .attribute_name("gsi2sk")
                        .build(),
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi2pk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi2sk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();
}

#[derive(Debug, Deez, Clone)]
pub struct Baz {
    pub baz_string_1: String,
    pub baz_string_2: String,
}

impl Default for Baz {
    fn default() -> Self {
        Baz {
            baz_string_1: "baz".to_string(),
            baz_string_2: "bazbaz".to_string(),
        }
    }
}

#[derive(Debug, Deez, Clone)]
pub struct Bar {
    pub bar_string_1: String,
    pub bar_string_2: String,
    pub baz_1: Baz,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            bar_string_1: "bar".to_string(),
            bar_string_2: "barbar".to_string(),
            baz_1: Baz {
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deez, Default)]
#[deez_schema(table = "foo_table", service = "foo_service", entity = "foo_entity")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
pub struct Foo {
    #[deez_primary(key = "hash")]
    pub string_1: Option<String>,
    pub bar_1: Option<Bar>,
}

#[derive(Debug, Deez)]
#[deez_schema(table = "foo_table", service = "foo_service", entity = "task")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "foo_gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2_name = "foo_gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Task {
    #[deez_primary(key = "hash")]
    #[deez_gsi1(key = "range", position = 1)]
    #[deez_gsi2(key = "range", position = 1)]
    pub task_id: Option<String>,
    #[deez_primary(key = "range", position = 1)]
    #[deez_gsi1(key = "hash")]
    #[deez_gsi2(key = "range")]
    pub project: Option<String>,
    #[deez_primary(key = "range")]
    #[deez_gsi1(key = "range")]
    #[deez_gsi2(key = "hash")]
    pub employee: Option<String>,
    pub description: String,
    #[deez_ignore(ignore)]
    pub some_metadata: String,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            task_id: Some("123".to_string()),
            project: Some("wwe".to_string()),
            employee: Some("ddp".to_string()),
            description: "self high five!".to_string(),
            some_metadata: "it's true".to_string(),
        }
    }
}
