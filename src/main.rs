use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use deez_derive::Sugon;

trait DeezMaps {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
}

// #[derive(Debug)]
// struct Bar {
//     lmao: String,
// }

#[derive(Sugon, Debug)]
struct Foo {
    primary_id: String,
    deez: usize,
    ligma: bool,
    // ree: Bar,
}

fn main() {
    println!("Hello, world!");

    let a = Foo {
        primary_id: format!("AAA"),
        deez: 3,
        ligma: true
    };

    println!("{:?}", a.to_av_map());
}
