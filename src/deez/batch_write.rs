use crate::deez::{DeezEntity, DeezResult};
use crate::types::error::DeezError;
use crate::types::schema::{composed_key, get_composed_index, composed_index, DynamoType, Index, IndexKey, IndexKeysComposed};
use aws_sdk_dynamodb::operation::batch_write_item::builders::BatchWriteItemFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, PutRequest, WriteRequest};
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

impl super::Deez {
    pub fn batch_write(&self) -> DeezBatchWriteBuilder {
        DeezBatchWriteBuilder {
            client: &self.client,
            writes: HashMap::new(),
        }
    }
}
pub struct DeezBatchWriteBuilder<'a> {
    client: &'a Client,
    writes: HashMap<String, Vec<WriteRequest>>,
}

impl<'a> DeezBatchWriteBuilder<'a> {
    pub fn put<T: DeezEntity>(
        mut self,
        entities: Vec<&T>,
    ) -> DeezResult<DeezBatchWriteBuilder<'a>> {
        for entity in entities.iter() {
            let request = WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                        .set_item(Some(entity.to_av_map_with_keys()?))
                        .build(),
                )
                .build();
            if let Some(y) = self.writes.get_mut(entity.schema().table) {
                y.push(request);
            } else {
                self.writes
                    .insert(entity.schema().table.to_string(), vec![request]);
            }
        }
        Ok(self)
    }

    pub fn delete<T: DeezEntity>(
        mut self,
        entities: Vec<&T>,
    ) -> DeezResult<DeezBatchWriteBuilder<'a>> {
        for entity in entities.iter() {
            let a = get_composed_index!(entity, Index::Primary);
            let request = WriteRequest::builder()
                .delete_request(
                    DeleteRequest::builder()
                        .key(a.partition_key.0, AttributeValue::S(a.partition_key.1))
                        .key(a.sort_key.0, AttributeValue::S(a.sort_key.1))
                        .build(),
                )
                .build();
            if let Some(y) = self.writes.get_mut(entity.schema().table) {
                y.push(request);
            } else {
                self.writes
                    .insert(entity.schema().table.to_string(), vec![request]);
            }
        }
        Ok(self)
    }

    pub fn build(&self) -> DeezResult<BatchWriteItemFluentBuilder> {
        Ok(self
            .client
            .batch_write_item()
            .set_request_items(Some(self.writes.clone())))
    }
}

// todo: tests
