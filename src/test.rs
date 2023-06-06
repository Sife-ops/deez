#[cfg(test)]
mod tests {
    use crate::*;

    const TABLE_NAME: &str = "footable";
    const PRIMARY: Index = Index::Primary;
    const GSI1: Index = Index::Gsi1("gsi1");

    #[derive(DeezEntity, Debug, Default)]
    pub struct Foo {
        pub foo_string_1: String,
        pub foo_string_2: String,
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

        fn index_keys(&self) -> HashMap<Index, IndexKeys> {
            let mut m = HashMap::new();
            m.insert(
                PRIMARY,
                IndexKeys {
                    partition_key: Key {
                        field: "pk",
                        composite: vec!["foo_string_1".to_string()],
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
                        composite: vec!["foo_string_2".to_string()],
                    },
                    sort_key: Key {
                        field: "gsi1sk",
                        composite: vec![],
                    },
                },
            );
            m
        }

        fn generated() -> Self {
            Foo {
                ..Default::default()
            }
        }
    }

    // requires dynamodb-local with `docker run -p 8000:8000 amazon/dynamodb-local`,
    // then run with `cargo test int -- --nocapture --test-threads 1`
    mod integration {
        use super::*;
        use aws_sdk_dynamodb::types::{
            AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType,
            Projection, ProjectionType, ScalarAttributeType,
        };
        use aws_sdk_dynamodb::Client;

        async fn make_client() -> Client {
            Client::new(
                &aws_config::from_env()
                    .endpoint_url("http://localhost:8000")
                    .region("us-east-1")
                    .load()
                    .await,
            )
        }

        async fn make_deez() -> Deez {
            Deez::new(make_client().await)
        }

        // todo: initialize table

        #[tokio::test]
        async fn int_1_init_ddb() {
            let c = make_client().await;
            if let Ok(_) = c.delete_table().table_name(TABLE_NAME).send().await {}
            c.create_table()
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

        #[tokio::test]
        async fn int_2_put() {
            let d = make_deez().await;
            d.put(&Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: "bar".to_string(),
                foo_usize: 69,
                foo_bool: false,
            })
            .unwrap()
            .send()
            .await
            .unwrap();
        }

        #[tokio::test]
        async fn int_2_macro_put() {
            let d = make_deez().await;
            put_Foo!(
                d,
                Foo {
                    foo_string_1: "int_2_macro_put1".to_string(),
                    foo_string_2: "int_2_macro_put2".to_string(),
                    foo_usize: 69,
                    foo_bool: false,
                }
            );
        }

        #[tokio::test]
        async fn int_3() {
            let d = make_deez().await;
            let r = d
                .query(
                    PRIMARY,
                    &Foo {
                        foo_string_1: "foo".to_string(),
                        ..Default::default()
                    },
                )
                .unwrap()
                .send()
                .await
                .unwrap();
            let a = r.items().unwrap();
            let b = Foo::from_av_map_slice(a).unwrap();

            println!("{:#?}", b);
        }

        #[tokio::test]
        async fn int_4() {
            let d = make_deez().await;
            let _r = d
                .update(&Foo {
                    foo_string_1: "foo".to_string(),
                    foo_usize: 200,
                    ..Default::default()
                })
                .unwrap()
                .send()
                .await
                .unwrap();
        }
    }

    mod unit {
        use super::*;

        #[test]
        fn unit_1() {
            println!("unit");
            let a = Foo {
                foo_string_1: format!("bar"),
                foo_string_2: format!("baz"),
                foo_usize: 3,
                foo_bool: true,
                // foo_skip: format!("plz skip"),
            };

            ////////////////////////////////////////////////////////////////////////
            let b = a.to_av_map_keys().unwrap();
            println!("{:#?}", b);

            assert_eq!(
                b.get("foo_string").unwrap().as_s().unwrap().to_string(),
                "bar".to_string()
            );
            assert_eq!(b.get("foo_usize").unwrap().as_n().unwrap().to_string(), "3");

            ////////////////////////////////////////////////////////////////////////
            // let c = Foo::from_av_map(b);
            // println!("{:#?}", c);
        }
    }
}
