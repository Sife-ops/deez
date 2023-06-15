use crate::deez::{DeezEntity, DeezError, DeezResult};
use crate::types::schema::{
    composed_index, composed_key, get_composed_index, DynamoType, Index, IndexKey,
    IndexKeysComposed,
};
use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use std::collections::HashMap;

impl super::Deez {
    pub fn put(&self, entity: &impl DeezEntity) -> DeezResult<PutItemFluentBuilder> {
        let av_map = entity.to_av_map_with_keys()?;

        Ok(self
            .client
            .put_item()
            .table_name(entity.schema().table)
            .set_item(Some(av_map)))
    }

    pub fn create(&self, entity: &impl DeezEntity) -> DeezResult<PutItemFluentBuilder> {
        let primary_index = get_composed_index!(entity, Index::Primary);

        Ok(self
            .put(entity)?
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), primary_index.partition_key.0),
                ("#sk".to_string(), primary_index.sort_key.0),
            ]))))
    }
}

// todo: tests
#[cfg(test)]
mod tests {
    use crate::{mocks::mocks::*, DeezResult};

    #[tokio::test]
    async fn put_create() -> DeezResult<()> {
        let a = make_mock_deez().await;

        let b = Foo {
            foo_string_1: "aaa".to_string(),
            foo_string_2: "bbb".to_string(),
            foo_string_3: "ccc".to_string(),
            foo_string_4: "ddd".to_string(),
            foo_string_5: "eee".to_string(),
            foo_string_6: "fff".to_string(),
            foo_bool: true,
            ..Default::default()
        };

        let c = a.create(&b)?;

        println!("{:#?}", c);
        // todo: assert

        Ok(())
    }
}
