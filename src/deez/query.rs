use crate::{DeezEntity, DeezError, DeezResult, Index};
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl super::Deez {
    pub fn query(&self, index: Index, entity: &impl DeezEntity) -> DeezResult<DeezQueryBuilder> {
        let i = entity.get_composed_index(&index, &entity.to_av_map_with_keys()?)?;

        let mut query = self.client.query().table_name(entity.meta().table);
        if index != Index::Primary {
            query = query.index_name(index.to_string());
        }

        let mut names = HashMap::new();
        let mut values = HashMap::new();
        names.insert("#pk".to_string(), i.partition_key.field.clone());
        values.insert(":pk".to_string(), i.partition_key.value.clone());
        names.insert("#sk1".to_string(), i.sort_key.field.clone());
        values.insert(":sk1".to_string(), i.sort_key.value.clone());

        Ok(DeezQueryBuilder {
            index,
            query,
            exp: String::from("#pk = :pk"),
            exp_appendix: String::from("and begins_with(#sk1, :sk1)"), // default expression
            names,
            values,
        })
    }
}

pub struct DeezQueryBuilder {
    pub index: Index,
    pub query: QueryFluentBuilder,
    pub exp: String,
    pub exp_appendix: String,
    pub names: HashMap<String, String>,
    pub values: HashMap<String, AttributeValue>,
}

// todo: `where` clause
impl DeezQueryBuilder {
    // fn set_sk1(
    //     &self,
    //     m: &mut HashMap<String, AttributeValue>,
    //     v: AttributeValue,
    // ) -> DeezResult<()> {
    //     *m.get_mut(":sk1")
    //         .ok_or(DeezError::MapKey(":sk1".to_string()))? = v;
    //     Ok(())
    // }

    pub fn begins(mut self, entity: &impl DeezEntity) -> DeezResult<DeezQueryBuilder> {
        let i = entity.get_composed_index(&self.index, &entity.to_av_map_with_keys()?)?;
        *self
            .values
            .get_mut(":sk1")
            .ok_or(DeezError::MapKey(":sk1".to_string()))? = i.sort_key.value;
        Ok(self)
    }

    // todo: FilterExpression
    pub fn between(
        mut self,
        entity1: &impl DeezEntity,
        entity2: &impl DeezEntity,
    ) -> DeezResult<DeezQueryBuilder> {
        let i1 = entity1.get_composed_index(&self.index, &entity1.to_av_map_with_keys()?)?;
        let i2 = entity2.get_composed_index(&self.index, &entity2.to_av_map_with_keys()?)?;
        *self
            .values
            .get_mut(":sk1")
            .ok_or(DeezError::MapKey(":sk1".to_string()))? = i1.sort_key.value;
        self.values.insert(":sk2".to_string(), i2.sort_key.value);
        self.exp_appendix = String::from("and #sk1 BETWEEN :sk1 AND :sk2");
        Ok(self)
    }

    // todo: lte
    pub fn lt(mut self, entity: &impl DeezEntity) -> DeezResult<DeezQueryBuilder> {
        let i = entity.get_composed_index(&self.index, &entity.to_av_map_with_keys()?)?;
        *self
            .values
            .get_mut(":sk1")
            .ok_or(DeezError::MapKey(":sk1".to_string()))? = i.sort_key.value;
        self.exp_appendix = String::from("and #sk1 < :sk1");
        Ok(self)
    }

    // todo: gt
    pub fn gte(mut self, entity: &impl DeezEntity) -> DeezResult<DeezQueryBuilder> {
        let i = entity.get_composed_index(&self.index, &entity.to_av_map_with_keys()?)?;
        *self
            .values
            .get_mut(":sk1")
            .ok_or(DeezError::MapKey(":sk1".to_string()))? = i.sort_key.value;
        self.exp_appendix = String::from("and #sk1 >= :sk1");
        Ok(self)
    }

    // todo: execution options
    pub fn build(self) -> QueryFluentBuilder {
        self.query
            .key_condition_expression(format!("{} {}", self.exp, self.exp_appendix))
            .set_expression_attribute_names(Some(self.names))
            .set_expression_attribute_values(Some(self.values))
    }
}
