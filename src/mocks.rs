#[allow(unused)]
#[cfg(test)]
pub mod mocks {
    use crate::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;
    use aws_smithy_types::Blob;
    use std::collections::HashMap;

    pub async fn make_client() -> Client {
        Client::new(
            &aws_config::from_env()
                .endpoint_url("http://localhost:8000")
                .region("us-east-1")
                .load()
                .await,
        )
    }

    #[derive(Debug, Default, Deez)]
    pub struct Buz {
        #[deez_vec(dynamo_type = "list")]
        pub buz_vec_1: Vec<String>,
        #[deez_vec(dynamo_type = "list")]
        pub buz_vec_2: Vec<f64>,
        #[deez_vec(dynamo_type = "list")]
        pub buz_vec_3: Vec<Blob>,
        #[deez_vec(dynamo_type = "list")]
        pub buz_vec_4: Vec<Baz>,
        #[deez_vec(dynamo_type = "set")]
        pub buz_vec_5: Vec<String>,
        #[deez_vec(dynamo_type = "set")]
        pub buz_vec_6: Vec<f64>,
        #[deez_vec(dynamo_type = "set")]
        pub buz_vec_7: Vec<Blob>,
    }

    #[derive(Debug, Deez)]
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

    #[derive(Debug, Deez)]
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

    // todo: ignore snake case for this
    #[derive(Debug, Deez)]
    #[deez_schema(table = "foo_table", service = "foo_service", entity = "foo_entity")]
    #[deez_schema(primary_hash = "pk", primary_range = "sk")]
    #[deez_schema(gsi1_name = "foo_gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
    #[deez_schema(gsi2_name = "foo_gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
    pub struct Foo {
        #[deez_primary(key = "hash")]
        pub foo_string_1: String,
        #[deez_primary(key = "range")]
        pub foo_string_2: String,
        #[deez_primary(key = "range", position = 1)]
        pub foo_string_3: String,
        #[deez_gsi1(key = "hash")]
        pub foo_string_4: String,
        #[deez_gsi2(key = "hash")]
        pub foo_string_5: String,
        #[deez_ignore(ignore)]
        pub foo_string_6: String,
        #[deez_gsi1(key = "range")]
        pub foo_num1: f64,
        pub foo_bool1: bool,
        pub foo_nested_1: Bar,
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
                foo_nested_1: Bar {
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Deez, Clone)]
    #[deez_schema(table = "TaskTable", service = "TaskService", entity = "Task")]
    #[deez_schema(primary_hash = "pk", primary_range = "sk")]
    #[deez_schema(gsi1_name = "task_gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
    #[deez_schema(gsi2_name = "task_gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
    pub struct Task {
        #[deez_primary(key = "hash")]
        #[deez_gsi1(key = "range", position = 1)]
        #[deez_gsi2(key = "range", position = 1)]
        pub task_id: String,
        #[deez_primary(key = "range", position = 1)]
        #[deez_gsi1(key = "hash")]
        #[deez_gsi2(key = "range")]
        pub project: String,
        #[deez_primary(key = "range")]
        #[deez_gsi1(key = "range")]
        #[deez_gsi2(key = "hash")]
        pub employee: String,
        pub description: String,
        #[deez_ignore(ignore)]
        pub some_metadata: String,
    }

    impl Default for Task {
        fn default() -> Self {
            Task {
                task_id: "".to_string(),
                project: "".to_string(),
                employee: "".to_string(),
                description: "".to_string(),
                some_metadata: "".to_string(),
            }
        }
    }
}
