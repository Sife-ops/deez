use crate::{DeezEntity, DeezResult, Index};
use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use std::collections::HashMap;

impl super::Deez {
    pub fn create(&self, entity: &impl DeezEntity) -> DeezResult<PutItemFluentBuilder> {
        let av_map = entity.to_av_map_with_keys()?;
        let i = entity.get_composed_index(&Index::Primary, &av_map)?;
        Ok(self
            .client
            .put_item()
            .table_name(entity.meta().table)
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), i.partition_key.field),
                ("#sk".to_string(), i.sort_key.field),
            ])))
            .set_item(Some(av_map)))
    }
}
