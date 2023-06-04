use std::collections::HashMap;

use macro_deez_derive::Sugon;

trait MyTrait {
    fn to_avmap(&self) -> HashMap<String, String>;
}

#[derive(Sugon, Debug)]
struct Foo {
    primary_id: String,
    deez: String,
}

fn main() {
    println!("Hello, world!");

    let a = Foo {
        primary_id: format!("AAA"),
        deez: format!("lol")
    };

    println!("{:?}", a.to_avmap());
}
