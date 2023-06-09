use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

use crate::types::schema::{IndexKeysJoined, KeyJoined};
use crate::{DeezError, Index, IndexKeys, Meta};

pub type DeezResult<T> = Result<T, DeezError>;

pub struct Deez {
    client: Client,
}

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
