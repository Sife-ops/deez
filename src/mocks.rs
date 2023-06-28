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

    #[derive(Deez, Debug)]
    pub struct Buss {
        pub string: String,
        pub string_opt: Option<String>,
        #[deez_vec(dynamo_type = "list")]
        pub string_list: Vec<String>,
        #[deez_vec(dynamo_type = "list")]
        pub string_opt_list: Option<Vec<String>>,
        #[deez_vec(dynamo_type = "set")]
        pub string_set: Vec<String>,
        #[deez_vec(dynamo_type = "set")]
        pub string_opt_set: Option<Vec<String>>,

        pub num: f64,
        pub num_opt: Option<f64>,
        #[deez_vec(dynamo_type = "list")]
        pub num_list: Vec<f64>,
        #[deez_vec(dynamo_type = "list")]
        pub num_opt_list: Option<Vec<f64>>,
        #[deez_vec(dynamo_type = "set")]
        pub num_set: Vec<f64>,
        #[deez_vec(dynamo_type = "set")]
        pub num_opt_set: Option<Vec<f64>>,

        pub boolean: bool,
        pub boolean_opt: Option<bool>,
        #[deez_vec(dynamo_type = "list")]
        pub boolean_list: Vec<bool>,
        #[deez_vec(dynamo_type = "list")]
        pub boolean_opt_list: Option<Vec<bool>>,

        pub blob: Blob,
        pub blob_opt: Option<Blob>,
        #[deez_vec(dynamo_type = "list")]
        pub blob_list: Vec<Blob>,
        #[deez_vec(dynamo_type = "list")]
        pub blob_opt_list: Option<Vec<Blob>>,
        #[deez_vec(dynamo_type = "set")]
        pub blob_set: Vec<Blob>,
        #[deez_vec(dynamo_type = "set")]
        pub blob_opt_set: Option<Vec<Blob>>,

        pub bar: Bar,
        pub bar_opt: Option<Bar>,
        #[deez_vec(dynamo_type = "list")]
        pub bar_list: Vec<Bar>,
        #[deez_vec(dynamo_type = "list")]
        pub bar_opt_list: Option<Vec<Bar>>,
    }

    impl Default for Buss {
        fn default() -> Self {
            Buss {
                string: "a".to_string(),
                string_opt: Some("a".to_string()),
                string_list: vec!["a".to_string()],
                string_opt_list: Some(vec!["a".to_string()]),
                string_set: vec!["a".to_string()],
                string_opt_set: Some(vec!["a".to_string()]),

                num: 68.0,
                num_opt: Some(67.0),
                num_list: vec![66.0],
                num_opt_list: Some(vec![65.0]),
                num_set: vec![64.0],
                num_opt_set: Some(vec![70.0]),

                boolean: true,
                boolean_opt: Some(true),
                boolean_list: vec![true, false],
                boolean_opt_list: Some(vec![true, false]),

                blob: Blob::new([1, 2]),
                blob_opt: Some(Blob::new([3, 4])),
                blob_list: vec![Blob::new([3, 2])],
                blob_opt_list: Some(vec![Blob::new([2, 1])]),
                blob_set: vec![Blob::new([5, 3])],
                blob_opt_set: Some(vec![Blob::new([1, 2])]),

                bar: Bar::default(),
                bar_opt: Some(Bar::default()),
                bar_list: vec![Bar::default()],
                bar_opt_list: Some(vec![Bar::default()]),
            }
        }
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

    #[derive(Debug, Deez, Clone)]
    #[deez_schema(table = "TaskTable", service = "TaskService", entity = "Task")]
    #[deez_schema(primary_hash = "pk", primary_range = "sk")]
    #[deez_schema(gsi1_name = "task_gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
    #[deez_schema(gsi2_name = "task_gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
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
}
