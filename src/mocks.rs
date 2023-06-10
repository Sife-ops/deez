#[cfg(test)]
pub mod mocks {
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;
    use std::collections::HashMap;

    use crate::{
        Deez, DeezEntity, DeezEntityPartial, DeezError, DeezMeta, Index, IndexKeys, Key, Meta,
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
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
            }
        }

        fn indexes(&self) -> HashMap<Index, IndexKeys> {
            let mut m = HashMap::new();
            m.insert(
                Index::Primary,
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
            m
        }

        fn generated() -> Self {
            Foo {
                foo_usize: 33,
                ..Default::default()
            }
        }
    }
}
