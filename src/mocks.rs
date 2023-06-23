#[allow(unused)]
#[cfg(test)]
pub mod mocks {
    use crate::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::Client;
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

    #[derive(Debug, Deez)]
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
