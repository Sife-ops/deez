use crate::{DeezEntity, DeezError, DeezResult, DynamoType, Index, Schema};
use aws_sdk_dynamodb::{
    operation::update_item::builders::UpdateItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    // todo: patch
    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<DeezUpdateBuilder> {
        let i = entity.get_composed_index(&Index::Primary)?;
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
            exp_attr_names: HashMap::new(),
            exp_attr_values: HashMap::new(),
            schema: entity.schema(),
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
    pub exp_attr_names: HashMap<String, String>,
    pub exp_attr_values: HashMap<String, AttributeValue>,
    pub schema: Schema,
}

impl DeezUpdateBuilder {
    // todo: set_bool, set_number
    pub fn set_string(mut self, map: HashMap<String, String>) -> DeezResult<DeezUpdateBuilder> {
        match self.exp.len() {
            0 => self.exp.push_str("SET"),
            _ => self.exp.push_str(" SET"),
        }

        for (i, (update_key, update_value)) in map.iter().enumerate() {
            self.exp_attr_names
                .insert(format!("#{}", update_key), update_key.clone());

            let exp_value_var = self.make_exp_value_var(&self.exp_attr_values, update_key);

            match i {
                0 => self
                    .exp
                    .push_str(&format!(" #{} = :{}", update_key, exp_value_var)),
                _ => self
                    .exp
                    .push_str(&format!(", #{} = :{}", update_key, exp_value_var)),
            }

            match self
                .schema
                .attributes
                .get(update_key.as_str())
                .ok_or(DeezError::UnknownAttribute(update_key.to_string()))?
            {
                DynamoType::DynamoString => {
                    self.exp_attr_values.insert(
                        format!(":{}", exp_value_var),
                        AttributeValue::S(update_value.to_string()),
                    );
                }
                _ => panic!(), // todo: Err()
            }
        }

        Ok(self)
    }

    // todo: add_float
    pub fn add(mut self, map: HashMap<String, u64>) -> DeezResult<DeezUpdateBuilder> {
        match self.exp.len() {
            0 => self.exp.push_str("ADD"),
            _ => self.exp.push_str(" ADD"),
        }

        for (i, (update_key, update_value)) in map.iter().enumerate() {
            self.exp_attr_names
                .insert(format!("#{}", update_key), update_key.clone());

            let exp_value_var = self.make_exp_value_var(&self.exp_attr_values, update_key);

            match i {
                0 => self
                    .exp
                    .push_str(&format!(" #{} :{}", update_key, exp_value_var)),
                _ => self
                    .exp
                    .push_str(&format!(", #{} :{}", update_key, exp_value_var)),
            }

            match self
                .schema
                .attributes
                .get(update_key.as_str())
                .ok_or(DeezError::UnknownAttribute(update_key.to_string()))?
            {
                DynamoType::DynamoNumber(_) => {
                    self.exp_attr_values.insert(
                        format!(":{}", exp_value_var),
                        AttributeValue::N(update_value.to_string()),
                    );
                }
                _ => panic!(), // todo: Err()
            }
        }

        Ok(self)
    }

    pub fn build(self) -> UpdateItemFluentBuilder {
        self.builder
            .update_expression(self.exp)
            .set_expression_attribute_names(Some(self.exp_attr_names))
            .set_expression_attribute_values(Some(self.exp_attr_values))
    }

    fn make_exp_value_var(&self, a: &HashMap<String, AttributeValue>, b: &String) -> String {
        let c = a
            .iter()
            .filter(|(x, _)| x.starts_with(b))
            .collect::<HashMap<&String, &AttributeValue>>();
        format!("{}_u{}", b, c.len())
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
            .set_string(HashMap::from([
                ("foo_string_3".to_string(), "ccc".to_string()),
                ("foo_string_4".to_string(), "ddd".to_string()),
            ]))
            .unwrap();

        println!("{:#?}", u.exp);
        println!("{:#?}", u.exp_attr_names);
        println!("{:#?}", u.exp_attr_values);

        assert!(
            u.exp == "SET #foo_string_3 = :foo_string_3_u0, #foo_string_4 = :foo_string_4_u0"
                || u.exp
                    == "SET #foo_string_4 = :foo_string_4_u0, #foo_string_3 = :foo_string_3_u0"
        );
        assert_eq!(
            u.exp_attr_names.get("#foo_string_3").unwrap(),
            "foo_string_3"
        );
        assert_eq!(
            u.exp_attr_names.get("#foo_string_4").unwrap(),
            "foo_string_4"
        );
        assert_eq!(
            u.exp_attr_values.get(":foo_string_3_u0").unwrap(),
            &AttributeValue::S("ccc".to_string())
        );
        assert_eq!(
            u.exp_attr_values.get(":foo_string_4_u0").unwrap(),
            &AttributeValue::S("ddd".to_string())
        );
    }
}
