#[allow(unused)]
#[cfg(test)]
pub mod mocks {
    use crate::*;
    use aws_sdk_dynamodb::Client;

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
}
