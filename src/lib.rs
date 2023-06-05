mod error;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
pub use deez_derive::DeezEntity;
use error::DeezError;
use std::collections::HashMap;

pub struct Deez {
    client: Client,
}

#[derive(Debug)]
pub struct Meta<'a> {
    pub table: &'a str,
    pub service: &'a str,
    pub entity: &'a str,
}

#[derive(Debug)]
pub struct IndexKeys<'a> {
    pub partition_key: Key<'a>,
    pub sort_key: Key<'a>,
}

#[derive(Debug)]
pub struct Key<'a> {
    pub field: &'a str,
    pub composite: Vec<String>, // todo: Vec<&'a str>?
}

impl Key<'_> {
    fn _join_composite(
        &self,
        attrs: &HashMap<String, AttributeValue>,
    ) -> Result<String, DeezError> {
        let mut j = String::new();
        for c in self.composite.iter() {
            let a = attrs.get(c).ok_or(DeezError::MapKey(c.to_string()))?;
            let s = match a {
                AttributeValue::S(b) => b.to_string(),
                AttributeValue::N(b) => b.to_string(),
                AttributeValue::Bool(b) => b.to_string(),
                _ => return Err(DeezError::InvalidComposite(c.to_string())),
            };
            j.push_str(&format!("#{}_{}", c, s));
        }
        Ok(j)
    }
}

pub trait DeezMeta {
    fn meta(&self) -> Meta;
    fn index_keys(&self) -> HashMap<Index, IndexKeys>;
}

pub trait DeezEntity: DeezMeta {
    fn to_av_map(&self) -> Result<HashMap<String, AttributeValue>, DeezError>;
    fn from_av_map(m: HashMap<String, AttributeValue>) -> Result<Self, DeezError>
    where
        Self: Sized;
}

#[derive(Eq, Hash, PartialEq)]
pub enum Index<'a> {
    Primary,
    Gsi1(&'a str),
    Gsi2(&'a str),
    Gsi3(&'a str),
    Gsi4(&'a str),
    Gsi5(&'a str),
    Gsi6(&'a str),
    Gsi7(&'a str),
    Gsi8(&'a str),
    Gsi9(&'a str),
    Gsi10(&'a str),
    Gsi11(&'a str),
    Gsi12(&'a str),
    Gsi13(&'a str),
    Gsi14(&'a str),
    Gsi15(&'a str),
    Gsi16(&'a str),
    Gsi17(&'a str),
    Gsi18(&'a str),
    Gsi19(&'a str),
    Gsi20(&'a str),
}

impl std::fmt::Display for Index<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Primary => write!(f, "primary"),
            Index::Gsi1(x) => write!(f, "{}", x),
            Index::Gsi2(x) => write!(f, "{}", x),
            Index::Gsi3(x) => write!(f, "{}", x),
            Index::Gsi4(x) => write!(f, "{}", x),
            Index::Gsi5(x) => write!(f, "{}", x),
            Index::Gsi6(x) => write!(f, "{}", x),
            Index::Gsi7(x) => write!(f, "{}", x),
            Index::Gsi8(x) => write!(f, "{}", x),
            Index::Gsi9(x) => write!(f, "{}", x),
            Index::Gsi10(x) => write!(f, "{}", x),
            Index::Gsi11(x) => write!(f, "{}", x),
            Index::Gsi12(x) => write!(f, "{}", x),
            Index::Gsi13(x) => write!(f, "{}", x),
            Index::Gsi14(x) => write!(f, "{}", x),
            Index::Gsi15(x) => write!(f, "{}", x),
            Index::Gsi16(x) => write!(f, "{}", x),
            Index::Gsi17(x) => write!(f, "{}", x),
            Index::Gsi18(x) => write!(f, "{}", x),
            Index::Gsi19(x) => write!(f, "{}", x),
            Index::Gsi20(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMARY: Index = Index::Primary;

    #[derive(DeezEntity, Debug, Default)]
    struct Foo {
        foo_string: String,
        #[deez(rename = "fooz")]
        foo_usize: usize,
        foo_bool: bool,
        // todo: other types
    }

    impl DeezMeta for Foo {
        fn meta(&self) -> Meta {
            Meta {
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
            }
        }
        fn index_keys(&self) -> HashMap<Index, IndexKeys> {
            let mut m = HashMap::new();
            m.insert(
                PRIMARY,
                IndexKeys {
                    partition_key: Key {
                        field: "pk",
                        composite: vec!["foo_string".to_string()],
                    },
                    sort_key: Key {
                        field: "sk",
                        composite: vec!["fooz".to_string()],
                    },
                },
            );
            m
        }
    }

    #[test]
    fn t1() {
        let a = Foo {
            foo_string: format!("bar"),
            foo_usize: 3,
            foo_bool: true,
        };

        ////////////////////////////////////////////////////////////////////////
        let b = a.to_av_map().unwrap();
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
