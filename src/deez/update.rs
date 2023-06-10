use crate::{DeezEntity, DeezEntityPartial, DeezResult, Index};
use aws_sdk_dynamodb::{
    operation::update_item::builders::UpdateItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<DeezUpdateBuilder> {
        let i = entity.get_composed_index(&Index::Primary, &entity.to_av_map_with_keys()?)?;
        let request = self
            .client
            .update_item()
            .table_name(entity.meta().table)
            .set_key(Some(HashMap::from([
                (i.partition_key.field, i.partition_key.value),
                (i.sort_key.field, i.sort_key.value),
            ])));

        Ok(DeezUpdateBuilder {
            builder: request,
            exp: String::new(),
            names: HashMap::new(),
            values: HashMap::new(),
        })
    }

    // todo: add
    // todo: subtract
    // todo: remove
}

#[derive(Debug)]
pub struct DeezUpdateBuilder {
    pub builder: UpdateItemFluentBuilder,
    pub exp: String,
    pub names: HashMap<String, String>,
    pub values: HashMap<String, AttributeValue>,
}

impl DeezUpdateBuilder {
    pub fn set(mut self, entity: &impl DeezEntityPartial) -> DeezResult<DeezUpdateBuilder> {
        match self.exp.len() {
            0 => self.exp.push_str("SET"),
            _ => self.exp.push_str(" SET"),
        }

        let m = entity.to_av_map();
        for (i, (a, b)) in m.iter().enumerate() {
            self.names.insert(format!("#{}", a), a.clone());
            self.values.insert(format!(":{}", a), b.clone());
            match i {
                0 => self.exp.push_str(&format!(" #{} = :{}", a, a)),
                _ => self.exp.push_str(&format!(", #{} = :{}", a, a)),
            }
        }

        Ok(self)
    }

    pub fn build(self) -> UpdateItemFluentBuilder {
        self.builder
            .update_expression(self.exp)
            .set_expression_attribute_names(Some(self.names))
            .set_expression_attribute_values(Some(self.values))
    }
}

#[cfg(test)]
mod tests {
    use aws_sdk_dynamodb::types::AttributeValue;

    use crate::mocks::mocks::*;

    #[tokio::test]
    async fn update_builder() {
        let d = make_mock_deez().await;

        let u = d
            .update(&Foo {
                foo_string_1: "aaa".to_string(),
                foo_string_2: "bbb".to_string(),
                ..Default::default()
            })
            .unwrap()
            .set(&FooPartial {
                foo_string_3: Some("ccc".to_string()),
                foo_string_4: Some("ddd".to_string()),
                ..Default::default()
            })
            .unwrap();

        assert!(
            u.exp == "SET #foo_string_3 = :foo_string_3, #foo_string_4 = :foo_string_4"
                || u.exp == "SET #foo_string_4 = :foo_string_4, #foo_string_3 = :foo_string_3"
        );
        assert_eq!(u.names.get("#foo_string_3").unwrap(), "foo_string_3");
        assert_eq!(u.names.get("#foo_string_4").unwrap(), "foo_string_4");
        assert_eq!(
            u.values.get(":foo_string_3").unwrap(),
            &AttributeValue::S("ccc".to_string())
        );
        assert_eq!(
            u.values.get(":foo_string_4").unwrap(),
            &AttributeValue::S("ddd".to_string())
        );
    }
}
