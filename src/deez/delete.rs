use crate::{DeezEntity, DeezResult, Index};
use aws_sdk_dynamodb::operation::delete_item::builders::DeleteItemFluentBuilder;
use std::collections::HashMap;

impl super::Deez {
    fn delete_(&self, entity: &impl DeezEntity) -> DeleteItemFluentBuilder {
        self.client.delete_item().table_name(entity.meta().table)
    }

    pub fn delete(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let i = entity.get_composed_index(&Index::Primary, &entity.to_av_map_with_keys()?)?;
        Ok(self.delete_(entity).set_key(Some(HashMap::from([
            (i.partition_key.field, i.partition_key.value),
            (i.sort_key.field, i.sort_key.value),
        ]))))
    }

    pub fn remove(&self, entity: &impl DeezEntity) -> DeezResult<DeleteItemFluentBuilder> {
        let i = entity.get_composed_index(&Index::Primary, &entity.to_av_map_with_keys()?)?;
        Ok(self
            .delete_(entity)
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), i.partition_key.field.clone()),
                ("#sk".to_string(), i.sort_key.field.clone()),
            ])))
            .set_key(Some(HashMap::from([
                (i.partition_key.field, i.partition_key.value),
                (i.sort_key.field, i.sort_key.value),
            ]))))
    }
}
