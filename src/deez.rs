use crate::Deez;
use aws_sdk_dynamodb::Client;

impl Deez {
    pub fn new(c: Client) -> Deez {
        Deez { client: c }
    }

    // todo: scan
    // todo: batch get
}

mod create;

mod batch_write;

mod delete;

mod query;

mod update;
