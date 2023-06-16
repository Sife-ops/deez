use crate::deez::{DeezEntity, DeezResult};
use crate::types::error::DeezError;
use crate::types::schema::{
    composed_index, composed_key, get_composed_index, DynamoType, Index, IndexKey,
    IndexKeysComposed,
};
use aws_sdk_dynamodb::operation::batch_write_item::builders::BatchWriteItemFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, PutRequest, WriteRequest};
use std::collections::HashMap;

impl super::Deez {
    pub fn batch_write(&self) -> DeezBatchWriteBuilder {
        let builder = self.client.batch_write_item();

        DeezBatchWriteBuilder {
            builder,
            writes: HashMap::new(),
        }
    }
}
pub struct DeezBatchWriteBuilder {
    builder: BatchWriteItemFluentBuilder,
    writes: HashMap<String, Vec<WriteRequest>>,
}

impl DeezBatchWriteBuilder {
    pub fn put<T: DeezEntity>(mut self, entities: Vec<&T>) -> DeezResult<DeezBatchWriteBuilder> {
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

    pub fn delete<T: DeezEntity>(mut self, entities: Vec<&T>) -> DeezResult<DeezBatchWriteBuilder> {
        for entity in entities.iter() {
            let composed = get_composed_index!(entity, Index::Primary);

            let request = WriteRequest::builder()
                .delete_request(
                    DeleteRequest::builder()
                        .key(
                            composed.partition_key.0,
                            AttributeValue::S(composed.partition_key.1),
                        )
                        .key(composed.sort_key.0, AttributeValue::S(composed.sort_key.1))
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

    pub fn build(self) -> DeezResult<BatchWriteItemFluentBuilder> {
        Ok(self.builder.set_request_items(Some(self.writes)))
    }
}
