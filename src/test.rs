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
                ..Default::default()
            }
        }
    }

    // requires dynamodb-local with `docker run -p 8000:8000 amazon/dynamodb-local`,
    // then run with `cargo test int -- --nocapture --test-threads 1`
    mod integration {
        use super::*;
        use chrono::prelude::*;

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

        // #[tokio::test]
        async fn init_table_1() {
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
        async fn int_1_create() {
            init_table_1().await;
            let d = make_deez().await;
            d.create(&Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: "bar".to_string(),
                foo_string_3: "baz".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 2, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                foo_usize: 69,
                foo_bool: false,
            })
            .unwrap()
            .send()
            .await
            .unwrap();
            // todo: assert?
        }

        #[tokio::test]
        async fn int_1_create_macro() {
            init_table_1().await;
            let d = make_deez().await;
            create_Foo!(
                d,
                Foo {
                    foo_string_1: "foo".to_string(),
                    foo_string_2: "bar".to_string(),
                    foo_string_3: "baz".to_string(),
                    foo_string_4: "buz".to_string(),
                    foo_usize: 69,
                    foo_bool: false,
                }
            );
        }

        #[tokio::test]
        async fn int_1_batch_write_delete() {
            init_table_1().await;
            let d = make_deez().await;
            d.batch_write()
                .put(vec![
                    &Foo {
                        foo_string_1: "foo".to_string(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "bar".to_string(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "baz".to_string(),
                        ..Default::default()
                    },
                ])
                .unwrap()
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();
            d.batch_write()
                .put(vec![&Foo {
                    foo_string_1: "fooz".to_string(),
                    ..Default::default()
                }])
                .unwrap()
                .delete(vec![
                    &Foo {
                        foo_string_1: "foo".to_string(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "bar".to_string(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "baz".to_string(),
                        ..Default::default()
                    },
                ])
                .unwrap()
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();
        }

        #[tokio::test]
        async fn int_1_query() {
            init_table_1().await;
            let d = make_deez().await;
            d.batch_write()
                .put(vec![
                    &Foo {
                        foo_string_1: "foo".to_string(),
                        foo_string_2: "deez".to_string(),
                        foo_string_3: "sugon".to_string(),
                        // foo_string_4: "1".to_string(),
                        foo_string_4: Utc
                            .with_ymd_and_hms(2015, 1, 2, 3, 4, 5)
                            .unwrap()
                            .to_rfc3339(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "bar".to_string(),
                        foo_string_2: "deez".to_string(),
                        foo_string_3: "sugon".to_string(),
                        // foo_string_4: "2".to_string(),
                        foo_string_4: Utc
                            .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
                            .unwrap()
                            .to_rfc3339(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "baz".to_string(),
                        foo_string_2: "deez".to_string(),
                        foo_string_3: "sugon".to_string(),
                        // foo_string_4: "3".to_string(),
                        foo_string_4: Utc
                            .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
                            .unwrap()
                            .to_rfc3339(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "for".to_string(),
                        foo_string_2: "deez".to_string(),
                        foo_string_3: "sugon".to_string(),
                        // foo_string_4: "4".to_string(),
                        foo_string_4: Utc
                            .with_ymd_and_hms(2015, 1, 5, 3, 4, 5)
                            .unwrap()
                            .to_rfc3339(),
                        ..Default::default()
                    },
                    &Foo {
                        foo_string_1: "far".to_string(),
                        foo_string_2: "deez".to_string(),
                        foo_string_3: "sugon".to_string(),
                        // foo_string_4: "5".to_string(),
                        foo_string_4: Utc
                            .with_ymd_and_hms(2015, 1, 6, 3, 4, 5)
                            .unwrap()
                            .to_rfc3339(),
                        ..Default::default()
                    },
                ])
                .unwrap()
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();

            let a = d
                .query(
                    PRIMARY,
                    &Foo {
                        foo_string_1: "foo".to_string(),
                        ..Default::default()
                    },
                )
                .unwrap()
                .build()
                .send()
                .await
                .unwrap();
            let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
            println!("{:#?}", b);
            // todo: assert

            let c = d
                .query(
                    GSI1,
                    &Foo {
                        foo_string_2: "deez".to_string(),
                        ..Default::default()
                    },
                )
                .unwrap()
                .gte(&Foo {
                    foo_string_3: "sugon".to_string(),
                    // foo_string_4: "3".to_string(),
                    foo_string_4: Utc
                        .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
                        .unwrap()
                        .to_rfc3339(),
                    ..Default::default()
                })
                .unwrap()
                .build()
                .send()
                .await
                .unwrap();
            let d = Foo::from_av_map_slice(c.items().unwrap()).unwrap();
            println!("{:#?}", d);
            // todo: assert
        }

        #[tokio::test]
        async fn int_4_update() {
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
                foo_string_1: format!("foo"),
                foo_string_2: format!("bar"),
                foo_string_3: "baz".to_string(),
                foo_string_4: "asdf".to_string(),
                foo_usize: 3,
                foo_bool: true,
                // foo_skip: format!("plz skip"),
            };

            ////////////////////////////////////////////////////////////////////////
            let b = a.to_av_map_with_keys().unwrap();
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
