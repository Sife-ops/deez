use crate::{DeezEntity, DeezResult, Index};
use aws_sdk_dynamodb::{
    operation::delete_item::builders::DeleteItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    fn delete_(&self, entity: &impl DeezEntity) -> DeleteItemFluentBuilder {
        self.client.delete_item().table_name(entity.schema().table)
    }

    pub fn delete(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let i = entity.get_composed_index(&Index::Primary)?;
        Ok(self.delete_(entity).set_key(Some(HashMap::from([
            (i.partition_key.0, AttributeValue::S(i.partition_key.1)),
            (i.sort_key.0, AttributeValue::S(i.sort_key.1)),
        ]))))
    }

    pub fn remove(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let i = entity.get_composed_index(&Index::Primary)?;
        Ok(self
            .delete_(entity)
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), i.partition_key.0.clone()),
                ("#sk".to_string(), i.sort_key.0.clone()),
            ])))
            .set_key(Some(HashMap::from([
                (i.partition_key.0, AttributeValue::S(i.partition_key.1)),
                (i.sort_key.0, AttributeValue::S(i.sort_key.1)),
            ]))))
    }
}
