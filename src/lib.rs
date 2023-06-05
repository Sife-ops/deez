mod error;

use aws_sdk_dynamodb::types::AttributeValue;
pub use deez_derive::DeezMaps;
use error::DeezError;
use std::collections::HashMap;

pub trait DeezMaps {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
    fn from_av_map(m: HashMap<String, AttributeValue>) -> Result<Self, DeezError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(DeezMaps, Debug, Default)]
    struct Foo {
        foo_string: String,
        #[deez(rename = "fooz")]
        foo_usize: usize,
        foo_bool: bool,
        // todo: other types
    }

    // // // fn ligma(a: &HashMap<String, AttributeValue>) -> Result<(), &AttributeValue> {
    // fn ligma(a: &HashMap<String, AttributeValue>) -> Result<(), DeezError> {
    //     a.get("foo_string").ok_or(DeezError::MapKey(format!("lol")))?.as_s()?.to_string();
    //     let b = "32".to_string();
    //     // let c = b.parse::<usize>()?;
    //     Ok(())
    // }

    #[test]
    fn t1() {
        let a = Foo {
            foo_string: format!("bar"),
            foo_usize: 3,
            foo_bool: true,
        };

        ////////////////////////////////////////////////////////////////////////
        let b = a.to_av_map();
        println!("{:#?}", b);

        assert_eq!(
            b.get("foo_string").unwrap().as_s().unwrap().to_string(),
            "bar".to_string()
        );
        assert_eq!(b.get("fooz").unwrap().as_n().unwrap().to_string(), "3");

        ////////////////////////////////////////////////////////////////////////
        let c = Foo::from_av_map(b);
        println!("{:#?}", c);
    }
}
