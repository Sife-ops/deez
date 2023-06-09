use crate::{DeezEntity, DeezResult, Index};
use aws_sdk_dynamodb::operation::update_item::builders::UpdateItemFluentBuilder;
use std::collections::HashMap;

impl super::Deez {
    // todo: update builder
    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<UpdateItemFluentBuilder> {
        let mut update_expression = String::from("SET");
        let av_map = entity.to_av_map();
        av_map.iter().enumerate().for_each(|(i, v)| match i {
            0 => update_expression.push_str(&format!(" #{} = :{}", v.0, v.0)),
            _ => update_expression.push_str(&format!(", #{} = :{}", v.0, v.0)),
        });

        let i = entity.get_composed_index(&Index::Primary, &entity.to_av_map_with_keys()?)?;
        let mut request = self
            .client
            .update_item()
            .table_name(entity.meta().table)
            .update_expression(update_expression)
            .set_key(Some(HashMap::from([
                (i.partition_key.field, i.partition_key.value),
                (i.sort_key.field, i.sort_key.value),
            ])));

        for (k, v) in av_map.iter() {
            request = request.expression_attribute_names(format!("#{}", k), k);
            request = request.expression_attribute_values(format!(":{}", k), v.clone());
        }

        Ok(request)
    }
}
