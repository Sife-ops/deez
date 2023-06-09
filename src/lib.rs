mod deez;
mod error;
mod test;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
pub use deez_derive::DeezEntity;
pub use error::DeezError;
use std::collections::HashMap;

type DeezResult<T> = Result<T, DeezError>;

pub struct Deez {
    client: Client, // todo: arc?
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
    pub composite: Vec<&'a str>, // todo: Vec<&'a str>?
}

impl Key<'_> {
    pub fn join_composite(&self, attrs: &HashMap<String, AttributeValue>) -> DeezResult<String> {
        let mut j = String::new();
        for c in self.composite.iter() {
            let a = attrs
                .get(&c.to_string())
                .ok_or(DeezError::MapKey(c.to_string()))?;
            let s = match a {
                AttributeValue::S(b) => b.to_string(),
                _ => return Err(DeezError::InvalidComposite(c.to_string())),
            };
            if s.len() < 1 {
                return Ok(j);
            }
            j.push_str(&format!("#{}_{}", c, s));
        }
        Ok(j)
    }
}

#[derive(Debug)]
pub struct IndexKeysJoined {
    pub partition_key: KeyJoined,
    pub sort_key: KeyJoined,
}

#[derive(Debug)]
pub struct KeyJoined {
    pub field: String,
    pub value: AttributeValue,
}

pub trait DeezMeta {
    fn meta(&self) -> Meta;
    fn indexes(&self) -> HashMap<Index, IndexKeys>;
    fn generated() -> Self;
}

pub trait DeezEntity: DeezMeta {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
    fn to_av_map_with_keys(&self) -> DeezResult<HashMap<String, AttributeValue>>;
    fn from_av_map(m: &HashMap<String, AttributeValue>) -> DeezResult<Self>
    where
        Self: Sized;

    fn from_av_map_slice(ms: &[HashMap<String, AttributeValue>]) -> DeezResult<Vec<Self>>
    where
        Self: Sized,
    {
        let mut v = Vec::new();
        for a in ms.iter() {
            v.push(Self::from_av_map(a)?)
        }
        Ok(v)
    }

    fn get_composed_index(
        &self,
        index: &Index,
        av_map: &HashMap<String, AttributeValue>,
    ) -> DeezResult<IndexKeysJoined> {
        let indexes = self.indexes();
        let index_keys = indexes
            .get(&index)
            .ok_or(DeezError::UnknownIndex(index.to_string()))?;
        let pkf = index_keys.partition_key.field;
        let skf = index_keys.sort_key.field;
        // let av_map = self.to_av_map_with_keys()?;
        Ok(IndexKeysJoined {
            partition_key: KeyJoined {
                field: pkf.to_string(),
                value: av_map
                    .get(pkf)
                    .ok_or(DeezError::MapKey(pkf.to_string()))?
                    .clone(),
            },
            sort_key: KeyJoined {
                field: skf.to_string(),
                value: av_map
                    .get(skf)
                    .ok_or(DeezError::MapKey(skf.to_string()))?
                    .clone(),
            },
        })
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum Index {
    Primary,
    Gsi1(&'static str),
    Gsi2(&'static str),
    Gsi3(&'static str),
    Gsi4(&'static str),
    Gsi5(&'static str),
    Gsi6(&'static str),
    Gsi7(&'static str),
    Gsi8(&'static str),
    Gsi9(&'static str),
    Gsi10(&'static str),
    Gsi11(&'static str),
    Gsi12(&'static str),
    Gsi13(&'static str),
    Gsi14(&'static str),
    Gsi15(&'static str),
    Gsi16(&'static str),
    Gsi17(&'static str),
    Gsi18(&'static str),
    Gsi19(&'static str),
    Gsi20(&'static str),
}

impl std::fmt::Display for Index {
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
