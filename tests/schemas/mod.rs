pub mod foo;
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
