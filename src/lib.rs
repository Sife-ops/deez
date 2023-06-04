use aws_sdk_dynamodb::types::AttributeValue;
pub use deez_derive::DeezMaps;
use std::collections::HashMap;

pub trait DeezMaps {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(DeezMaps, Debug)]
    struct Foo {
        foo_string: String,
        foo_usize: usize,
        foo_bool: bool,
        // todo: other types
    }

    #[test]
    fn t1() {
        let a = Foo {
            foo_string: format!("bar"),
            foo_usize: 3,
            foo_bool: true,
        };

        let b = a.to_av_map();
        println!("{:?}", b);

        assert_eq!(
            b.get("foo_string").unwrap().as_s().unwrap().to_string(),
            "bar".to_string()
        );
        // todo: test cases
    }
}
