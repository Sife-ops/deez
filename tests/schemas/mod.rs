use aws_sdk_dynamodb::Client;
use deez::Deez;
use std::sync::Arc;

pub mod foo;

pub async fn make_client() -> Client {
    Client::new(
        &aws_config::from_env()
            .endpoint_url("http://localhost:8000")
            .region("us-east-1")
            .load()
            .await,
    )
}

pub async fn make_deez() -> Deez {
    Deez::new(make_client().await)
}

pub async fn make_deez_arc() -> Arc<Deez> {
    Arc::new(Deez::new(make_client().await))
}
