#[cfg(test)]
pub mod mocks {
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;
    use std::collections::HashMap;

    use crate::{
        deez::DeezSchema, Deez, DeezEntity, DeezError, DynamoType, Index, IndexKey, IndexKeys, Key,
        Reflect, RustType, Schema,
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

    #[derive(DeezEntity, Debug, Reflect)]
    pub struct Foo {
        pub foo_string_1: String,
        pub foo_string_2: String,
        pub foo_string_3: String,
        pub foo_string_4: String,
        pub foo_string_5: String,
        pub foo_string_6: String,
        pub foo_isize: isize,
        pub foo_bool: bool,
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
                foo_isize: 69,
                foo_bool: false,
            }
        }
    }

    pub const GSI1: Index = Index::Gsi1("gsi1");
    pub const GSI2: Index = Index::Gsi2("gsi2");

    impl DeezSchema for Foo {
        fn schema(&self) -> Schema {
            Schema {
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
                primary_index: IndexKeys {
                    partition_key: IndexKey::Partition(Key {
                        field: "pk",
                        composite: vec!["foo_string_1"],
                    }),
                    sort_key: IndexKey::Sort(Key {
                        field: "sk",
                        composite: vec![],
                    }),
                },
                global_secondary_indexes: HashMap::from([
                    (
                        GSI1,
                        IndexKeys {
                            partition_key: IndexKey::Partition(Key {
                                field: "gsi1pk",
                                composite: vec!["foo_string_2"],
                            }),
                            sort_key: IndexKey::Sort(Key {
                                field: "gsi1sk",
                                composite: vec!["foo_string_1"],
                            }),
                        },
                    ),
                    (
                        GSI2,
                        IndexKeys {
                            partition_key: IndexKey::Partition(Key {
                                field: "gsi2pk",
                                composite: vec!["foo_string_3"],
                            }),
                            sort_key: IndexKey::Sort(Key {
                                field: "gsi2sk",
                                composite: vec!["foo_string_4", "foo_string_5"],
                            }),
                        },
                    ),
                ]),
                attributes: HashMap::from([
                    ("foo_string_1", DynamoType::DynamoString),
                    ("foo_string_2", DynamoType::DynamoString),
                    ("foo_string_3", DynamoType::DynamoString),
                    ("foo_string_4", DynamoType::DynamoString),
                    ("foo_string_5", DynamoType::DynamoString),
                    ("foo_string_6", DynamoType::DynamoString),
                    ("foo_isize", DynamoType::DynamoNumber(RustType::Isize)),
                    ("foo_bool", DynamoType::DynamoBool),
                ]),
            }
        }
    }
}
