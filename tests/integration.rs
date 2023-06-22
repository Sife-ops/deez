// requires dynamodb-local with `docker run -p 8000:8000 amazon/dynamodb-local`,
// then run with `cargo test --test integration -- --nocapture --test-threads 1`

mod schemas;
mod operations;
