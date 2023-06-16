use crate::deez::{DeezEntity, DeezError, DeezResult};
use crate::types::schema::{
    composed_index, composed_key, get_composed_index, DynamoType, Index, IndexKey,
    IndexKeysComposed,
};
use aws_sdk_dynamodb::{
    operation::delete_item::builders::DeleteItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    pub fn delete(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let primary_index = get_composed_index!(entity, Index::Primary);
        Ok(self
            .client
            .delete_item()
            .table_name(entity.schema().table)
            .set_key(Some(HashMap::from([
                (
                    primary_index.partition_key.0,
                    AttributeValue::S(primary_index.partition_key.1),
                ),
                (
                    primary_index.sort_key.0,
                    AttributeValue::S(primary_index.sort_key.1),
                ),
            ]))))
    }

    pub fn remove(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let primary_index = get_composed_index!(entity, Index::Primary);
        Ok(self
            .client
            .delete_item()
            .table_name(entity.schema().table)
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), primary_index.partition_key.0.to_string()),
                ("#sk".to_string(), primary_index.sort_key.0.to_string()),
            ])))
            .set_key(Some(HashMap::from([
                (
                    primary_index.partition_key.0,
                    AttributeValue::S(primary_index.partition_key.1),
                ),
                (
                    primary_index.sort_key.0,
                    AttributeValue::S(primary_index.sort_key.1),
                ),
            ]))))
    }
}
