use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, GlobalSecondaryIndex, KeySchemaElement,
    KeyType, Projection, ProjectionType, ScalarAttributeType,
};
use std::collections::HashMap;

use deez::{DeezEntity, DeezError, DeezMeta, Index, IndexKeys, Key, Meta};

use super::make_client;

pub const TABLE_NAME: &str = "footable";
pub const PRIMARY: Index = Index::Primary;
pub const GSI1: Index = Index::Gsi1("gsi1");

pub async fn init() {
    let client = make_client().await;
    if let Ok(_) = client.delete_table().table_name(TABLE_NAME).send().await {}
    client
        .create_table()
        .table_name(TABLE_NAME)
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

#[derive(DeezEntity, Debug, Default)]
pub struct Foo {
    pub foo_string_1: String,
    pub foo_string_2: String,
    pub foo_string_3: String,
    pub foo_string_4: String,
    pub foo_usize: u8,
    pub foo_bool: bool,
}

impl DeezMeta for Foo {
    fn meta(&self) -> Meta {
        Meta {
            table: TABLE_NAME,
            service: "fooservice",
            entity: "fooentity",
        }
    }

    fn indexes(&self) -> HashMap<Index, IndexKeys> {
        let mut m = HashMap::new();
        m.insert(
            PRIMARY,
            IndexKeys {
                partition_key: Key {
                    field: "pk",
                    composite: vec!["foo_string_1"],
                },
                sort_key: Key {
                    field: "sk",
                    composite: vec![],
                },
            },
        );
        m.insert(
            GSI1,
            IndexKeys {
                partition_key: Key {
                    field: "gsi1pk",
                    composite: vec!["foo_string_2"],
                },
                sort_key: Key {
                    field: "gsi1sk",
                    composite: vec!["foo_string_3", "foo_string_4"],
                },
            },
        );
        m
    }

    fn generated() -> Self {
        Foo {
            foo_usize: 33,
            ..Default::default()
        }
    }
}
