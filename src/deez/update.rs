use crate::{DeezEntity, DeezResult, DynamoType, Index, Schema};
use aws_sdk_dynamodb::{
    operation::update_item::builders::UpdateItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<DeezUpdateBuilder> {
        let i = entity.get_composed_index(&Index::Primary).unwrap();
        let request = self
            .client
            .update_item()
            .table_name(entity.schema().table)
            .set_key(Some(HashMap::from([
                (i.partition_key.0, AttributeValue::S(i.partition_key.1)),
                (i.sort_key.0, AttributeValue::S(i.sort_key.1)),
            ])));

        Ok(DeezUpdateBuilder {
            builder: request,
            exp: String::new(),
            names: HashMap::new(),
            values: HashMap::new(),
            schema: entity.schema(),
        })
    }

    // todo: add
    // todo: subtract
    // todo: remove
}

// #[derive(Debug)]
pub struct DeezUpdateBuilder {
    pub builder: UpdateItemFluentBuilder,
    pub exp: String,
    pub names: HashMap<String, String>,
    pub values: HashMap<String, AttributeValue>,
    pub schema: Schema,
}

impl DeezUpdateBuilder {
    pub fn set(mut self, m: HashMap<String, String>) -> DeezResult<DeezUpdateBuilder> {
        match self.exp.len() {
            0 => self.exp.push_str("SET"),
            _ => self.exp.push_str(" SET"),
        }

        for (i, (a, b)) in m.iter().enumerate() {
            self.names.insert(format!("#{}", a), a.clone());

            match i {
                0 => self.exp.push_str(&format!(" #{} = :{}", a, a)),
                _ => self.exp.push_str(&format!(", #{} = :{}", a, a)),
            }

            let c = self.schema.attributes.get(a.as_str()).unwrap();
            match c.dynamo_type {
                DynamoType::DynamoString => {
                    self.values
                        .insert(format!(":{}", a), AttributeValue::S(b.to_string()));
                }
                DynamoType::DynamoNumber => {
                    self.values
                        .insert(format!(":{}", a), AttributeValue::N(b.to_string()));
                }
                DynamoType::DynamoBool => {
                    if b == "true" {
                        self.values
                            .insert(format!(":{}", a), AttributeValue::Bool(true));
                    } else if b == "fale" {
                        self.values
                            .insert(format!(":{}", a), AttributeValue::Bool(false));
                    } else {
                        panic!();
                    }
                }
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
    use crate::mocks::mocks::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

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
            .set(HashMap::from([
                ("foo_string_3".to_string(), "ccc".to_string()),
                ("foo_string_4".to_string(), "ddd".to_string()),
            ]))
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
