use aws_sdk_dynamodb::types::{
    AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
    ProjectionType, ScalarAttributeType,
};
use deez::*;

use super::make_client;

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
                .index_name("gsi1")
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
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();
}

#[derive(Debug, Deez, Clone)]
#[deez_schema(table = "foo_table", service = "foo_service", entity = "foo_entity")]
#[deez_schema(hash = "pk", range = "sk")]
#[deez_schema(gsi1 = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2 = "gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Foo {
    #[deez_attribute(index = "primary", key = "hash")]
    pub foo_string_1: String,
    #[deez_attribute(index = "primary", key = "range")]
    pub foo_string_2: String,
    #[deez_attribute(index = "primary", key = "range", position = 1)]
    pub foo_string_3: String,
    #[deez_attribute(index = "gsi1", key = "hash")]
    pub foo_string_4: String,
    #[deez_attribute(index = "gsi2", key = "hash")]
    pub foo_string_5: String,
    #[deez_ignore(ignore)]
    pub foo_string_6: String,
    #[deez_attribute(index = "gsi1", key = "range")]
    pub foo_num1: f64,
    pub foo_bool1: bool,
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            foo_string_1: "".to_string(),
            foo_string_2: "".to_string(),
            foo_string_3: "".to_string(),
            foo_string_4: "".to_string(),
            foo_string_5: "".to_string(),
            foo_string_6: "".to_string(),
            foo_num1: 69.0,
            foo_bool1: true,
        }
    }
}
