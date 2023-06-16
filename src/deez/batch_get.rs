use crate::deez::{DeezEntity, DeezError, DeezResult};
use crate::types::schema::{
    composed_index, composed_key, get_composed_index, DynamoType, Index, IndexKey,
    IndexKeysComposed,
};
use aws_sdk_dynamodb::operation::batch_get_item::builders::BatchGetItemFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
use std::collections::HashMap;

impl super::Deez {
    pub fn batch_get<T: DeezEntity>(
        &self,
        entities: Vec<&T>,
    ) -> DeezResult<BatchGetItemFluentBuilder> {
        let mut table_reqs: HashMap<String, Vec<HashMap<String, AttributeValue>>> = HashMap::new();
        for b in entities {
            let table_name = b.schema().table;
            let primary_index = get_composed_index!(b, Index::Primary);

            let key = HashMap::from([
                (
                    primary_index.partition_key.0,
                    AttributeValue::S(primary_index.partition_key.1),
                ),
                (
                    primary_index.sort_key.0,
                    AttributeValue::S(primary_index.sort_key.1),
                ),
            ]);

            if let Some(keys) = table_reqs.get_mut(table_name) {
                keys.push(key);
            } else {
                table_reqs.insert(table_name.to_string(), vec![key]);
            }
        }

        let mut builder = self.client.batch_get_item();
        for (k, v) in table_reqs {
            builder =
                builder.request_items(k, KeysAndAttributes::builder().set_keys(Some(v)).build());
        }

        Ok(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::mocks::*;

    #[tokio::test]
    #[ignore]
    async fn batch_get() -> DeezResult<()> {
        let a = make_mock_deez().await;

        let b = Foo {
            foo_string_1: "aaa".to_string(),
            // foo_string_2: "bbb".to_string(),
            // foo_string_3: "ccc".to_string(),
            // foo_string_4: "ddd".to_string(),
            // foo_string_5: "eee".to_string(),
            // foo_string_6: "fff".to_string(),
            // foo_bool: true,
            ..Default::default()
        };

        let c = a.batch_get(vec![
            &b,
            &Foo {
                foo_string_1: "lol".to_string(),
                ..Default::default()
            },
        ])?;

        println!("{:#?}", c);

        Ok(())
    }
}
