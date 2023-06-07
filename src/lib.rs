mod error;
mod test;

use aws_sdk_dynamodb::operation::batch_write_item::builders::BatchWriteItemFluentBuilder;
use aws_sdk_dynamodb::operation::delete_item::builders::DeleteItemFluentBuilder;
use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::operation::update_item::builders::UpdateItemFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, PutRequest, WriteRequest};
use aws_sdk_dynamodb::Client;
pub use deez_derive::DeezEntity;
use error::DeezError;
use std::collections::HashMap;

type DeezResult<T> = Result<T, DeezError>;

pub struct Deez {
    client: Client, // todo: arc?
}

pub struct DeezBatchWriteBuilder<'a> {
    client: &'a Client,
    pub writes: HashMap<String, WriteRequest>,
}

impl<'a> DeezBatchWriteBuilder<'a> {
    pub fn put<T: DeezEntity>(mut self, entities: Vec<T>) -> DeezResult<DeezBatchWriteBuilder<'a>> {
        for entity in entities.iter() {
            let request = WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                        .set_item(Some(entity.to_av_map_with_keys()?))
                        .build(),
                )
                .build();
            self.writes.insert(entity.meta().table.to_string(), request);
        }
        Ok(self)
    }

    pub fn delete<T: DeezEntity>(
        mut self,
        entities: Vec<T>,
    ) -> DeezResult<DeezBatchWriteBuilder<'a>> {
        for entity in entities.iter() {
            let request = WriteRequest::builder()
                .delete_request(
                    DeleteRequest::builder()
                        .set_key(Some(entity.to_av_map_with_keys()?))
                        .build(),
                )
                .build();
            self.writes.insert(entity.meta().table.to_string(), request);
        }
        Ok(self)
    }

    pub fn build(&self) -> DeezResult<BatchWriteItemFluentBuilder> {
        let mut m: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        for (k, v) in self.writes.iter() {
            if let Some(y) = m.get_mut(k) {
                y.push(v.clone());
            } else {
                m.insert(k.to_string(), vec![v.clone()]);
            }
        }
        Ok(self.client.batch_write_item().set_request_items(Some(m)))
    }
}

impl Deez {
    pub fn new(c: Client) -> Self {
        Deez { client: c }
    }

    pub fn put(&self, entity: &impl DeezEntity) -> DeezResult<PutItemFluentBuilder> {
        Ok(self
            .client
            .put_item()
            .table_name(entity.meta().table)
            .set_item(Some(entity.to_av_map_with_keys()?)))
    }

    pub fn batch_write<T: DeezEntity>(&self) -> DeezBatchWriteBuilder {
        DeezBatchWriteBuilder {
            client: &self.client,
            writes: HashMap::new(),
        }
    }

    pub fn query(
        &self,
        index: Index,
        entity: &impl DeezEntity,
    ) -> DeezResult<QueryFluentBuilder> {
        let index_keys = entity.index_keys();
        let i = index_keys
            .get(&index)
            .ok_or(DeezError::MapKey(index.to_string()))?;
        let pkf = i.partition_key.field.clone();
        let skf = i.sort_key.field.clone();
        // todo: verify the index composites exist in av
        let av = entity.to_av_map_with_keys()?;

        let mut request = self
            .client
            .query()
            .table_name(entity.meta().table)
            .key_condition_expression(format!("#{pkf} = :{pkf} and begins_with(#{skf}, :{skf})"))
            .expression_attribute_names(format!("#{pkf}"), pkf)
            .expression_attribute_names(format!("#{skf}"), skf)
            .expression_attribute_values(format!(":{pkf}"), av.get(pkf).unwrap().clone())
            .expression_attribute_values(format!(":{skf}"), av.get(skf).unwrap().clone());

        if index != Index::Primary {
            request = request.index_name(index.to_string());
        }

        Ok(request)
    }

    // todo: scan

    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<UpdateItemFluentBuilder> {
        // look up entity's pk and sk field name
        let index_keys = entity.index_keys();
        let primary_index = index_keys
            .get(&Index::Primary)
            .ok_or(DeezError::UnknownKey(Index::Primary.to_string()))?;
        let pk_field = primary_index.partition_key.field;
        let sk_field = primary_index.sort_key.field;

        // build update expression from the AttributeValue map
        let av_map = entity.to_av_map();
        let mut update_expression = String::from("SET");
        av_map.iter().enumerate().for_each(|(i, v)| match i {
            0 => update_expression.push_str(&format!(" #{} = :{}", v.0, v.0)),
            _ => update_expression.push_str(&format!(", #{} = :{}", v.0, v.0)),
        });

        let av_map_keys = entity.to_av_map_with_keys()?;
        let mut request = self
            .client
            .update_item()
            .table_name(entity.meta().table)
            .key(
                pk_field,
                av_map_keys
                    .get(pk_field)
                    .ok_or(DeezError::MapKey(pk_field.to_string()))?
                    .clone(),
            )
            .key(
                sk_field,
                av_map_keys
                    .get(sk_field)
                    .ok_or(DeezError::MapKey(sk_field.to_string()))?
                    .clone(),
            )
            .update_expression(update_expression);

        for (k, _) in av_map.iter() {
            request = request.expression_attribute_names(format!("#{}", k), k);
        }
        for (k, v) in av_map.iter() {
            request = request.expression_attribute_values(format!(":{}", k), v.clone());
        }

        Ok(request)
    }

    pub fn delete(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let index_keys = entity.index_keys();
        let primary_index = index_keys
            .get(&Index::Primary)
            .ok_or(DeezError::UnknownKey(Index::Primary.to_string()))?;
        let pk_field = primary_index.partition_key.field;
        let sk_field = primary_index.sort_key.field;

        let av_map_keys = entity.to_av_map_with_keys()?;
        Ok(self
            .client
            .delete_item()
            .table_name(entity.meta().table)
            .key(
                pk_field,
                av_map_keys
                    .get(pk_field)
                    .ok_or(DeezError::MapKey(pk_field.to_string()))?
                    .clone(),
            )
            .key(
                sk_field,
                av_map_keys
                    .get(sk_field)
                    .ok_or(DeezError::MapKey(sk_field.to_string()))?
                    .clone(),
            ))
    }
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
    ) -> DeezResult<String> {
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
