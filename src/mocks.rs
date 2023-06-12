#[cfg(test)]
pub mod mocks {
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;
    use std::collections::HashMap;

    use crate::{
        deez::DeezSchema, Deez, DeezEntity, DeezError, DynamoType, Index, IndexKeys, Key, Reflect,
        RustType, Schema,
    };

    pub async fn make_mock_client() -> Client {
        Client::new(
            &aws_config::from_env()
                .endpoint_url("http://localhost:8000")
                .region("us-east-1")
                .load()
                .await,
        )
    }

    pub async fn make_mock_deez() -> Deez {
        Deez::new(make_mock_client().await)
    }

    #[derive(DeezEntity, Debug, Default, Reflect)]
    pub struct Foo {
        pub foo_string_1: String,
        pub foo_string_2: String,
        pub foo_string_3: String,
        pub foo_string_4: String,
        pub foo_usize: isize,
        pub foo_bool: bool,
    }

    pub const GSI1: Index = Index::Gsi1("gsi1");

    impl DeezSchema for Foo {
        fn schema(&self) -> Schema {
            Schema {
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
                primary_index: IndexKeys {
                    partition_key: Key {
                        field: "pk",
                        composite: vec!["foo_string_1"],
                    },
                    sort_key: Key {
                        field: "sk",
                        composite: vec![],
                    },
                },
                global_secondary_indexes: HashMap::from([(
                    GSI1,
                    IndexKeys {
                        partition_key: Key {
                            field: "gsi1pk",
                            composite: vec!["foo_string_2"],
                        },
                        sort_key: Key {
                            field: "gsi1sk",
                            composite: vec!["foo_string_1"],
                        },
                    },
                )]),
                attributes: HashMap::from([
                    ("foo_string_1", DynamoType::DynamoString),
                    ("foo_string_2", DynamoType::DynamoString),
                    ("foo_string_3", DynamoType::DynamoString),
                    ("foo_string_4", DynamoType::DynamoString),
                    ("foo_usize", DynamoType::DynamoNumber(RustType::Isize)),
                    ("foo_bool", DynamoType::DynamoBool),
                ]),
            }
        }
    }
}
