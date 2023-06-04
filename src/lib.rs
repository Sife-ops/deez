use aws_sdk_dynamodb::types::AttributeValue;
pub use deez_derive::ToMap;
use std::collections::HashMap;

pub trait DeezMaps {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(ToMap, Debug)]
    struct Foo {
        foo_string: String,
        #[deez(rename = "fooz")]
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
        assert_eq!(b.get("fooz").unwrap().as_n().unwrap().to_string(), "3");
    }
}
