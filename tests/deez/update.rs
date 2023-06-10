// use deez::{DeezEntity, DeezMeta};

use super::super::schemas::foo::init;
// use super::super::schemas::make_deez;

#[ignore]
#[tokio::test]
async fn create() {
    init().await;
    // let d = make_deez().await;
}
