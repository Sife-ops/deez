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
            schema: entity.schema(),
            av_map: entity.to_av_map()?,
            exp: String::new(),
            exp_attr_names: HashMap::new(),
            exp_attr_values: HashMap::new(),
        })
    }
}

#[derive(Debug)]
pub struct DeezUpdateBuilder {
    pub builder: UpdateItemFluentBuilder,
    pub schema: Schema,
    pub av_map: HashMap<String, AttributeValue>,
    pub exp: String,
    pub exp_attr_names: HashMap<String, String>,
    pub exp_attr_values: HashMap<String, AttributeValue>,
}

macro_rules! sync_key {
    ($index_key: expr, $update_key: ident, $self: ident) => {
        for composite in $index_key.composite() {
            if composite == $update_key {
                let field = $index_key.field();
                let composed = $index_key.composed_key(&$self.av_map, &$self.schema)?;
                $self.exp.push_str(&format!(", #{} = :{}", field, field));
                $self
                    .exp_attr_names
                    .insert(format!("#{}", field), field.to_string());
                $self
                    .exp_attr_values
                    .insert(format!(":{}", field), AttributeValue::S(composed));
            }
        }
    };
}

macro_rules! unique_exp_value_var {
    ($a: expr, $b: ident) => {{
        let c = $a
            .iter()
            .filter(|(x, _)| x.starts_with(&format!(":{}", $b)))
            .collect::<HashMap<&String, &AttributeValue>>();
        format!("{}_u{}", $b, c.len())
    }};
}

impl DeezUpdateBuilder {
    // todo: set_bool, set_number
    pub fn set_string(mut self, map: HashMap<String, String>) -> DeezResult<DeezUpdateBuilder> {
        match self.exp.len() {
            0 => self.exp.push_str("SET"),
            _ => self.exp.push_str(" SET"),
        }

        for (update_key, update_value) in map.iter() {
            *self.av_map.get_mut(update_key).unwrap() = AttributeValue::S(update_value.to_string());
        }

        for (i, (update_key, update_value)) in map.iter().enumerate() {
            self.exp_attr_names
                .insert(format!("#{}", update_key), update_key.clone());

            let exp_value_var = unique_exp_value_var!(self.exp_attr_values, update_key);

            match i {
                0 => self
                    .exp
                    .push_str(&format!(" #{} = :{}", update_key, exp_value_var)),
                _ => self
                    .exp
                    .push_str(&format!(", #{} = :{}", update_key, exp_value_var)),
            }

            for (_, index_keys) in self.schema.global_secondary_indexes.iter() {
                sync_key!(index_keys.partition_key, update_key, self);
                sync_key!(index_keys.sort_key, update_key, self);
            }

            // todo: string-only composites
            self.exp_attr_values.insert(
                format!(":{}", exp_value_var),
                AttributeValue::S(update_value.to_string()),
            );
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

            let exp_value_var = unique_exp_value_var!(self.exp_attr_values, update_key);

            match i {
                0 => self
                    .exp
                    .push_str(&format!(" #{} :{}", update_key, exp_value_var)),
                _ => self
                    .exp
                    .push_str(&format!(", #{} :{}", update_key, exp_value_var)),
            }

            self.exp_attr_values.insert(
                format!(":{}", exp_value_var),
                AttributeValue::N(update_value.to_string()),
            );
        }

        Ok(self)
    }

    // todo: subtract
    // todo: remove

    pub fn build(self) -> UpdateItemFluentBuilder {
        self.builder
            .update_expression(self.exp)
            .set_expression_attribute_names(Some(self.exp_attr_names))
            .set_expression_attribute_values(Some(self.exp_attr_values))
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
                ..Default::default()
            })
            .unwrap()
            .set_string(HashMap::from([
                ("foo_string_2".to_string(), "rofl".to_string()),
                ("foo_string_6".to_string(), "lol".to_string()),
            ]))
            .unwrap()
            .set_string(HashMap::from([(
                "foo_string_6".to_string(),
                "lmao".to_string(),
            )]))
            .unwrap();

        // println!("{:#?}", u);
        println!("{:#?}", u.exp);
        println!("{:#?}", u.exp_attr_names);
        println!("{:#?}", u.exp_attr_values);

        // assert!(
        //     u.exp == "SET #foo_string_3 = :foo_string_3_u0, #foo_string_4 = :foo_string_4_u0"
        //         || u.exp
        //             == "SET #foo_string_4 = :foo_string_4_u0, #foo_string_3 = :foo_string_3_u0"
        // );
        // assert_eq!(
        //     u.exp_attr_names.get("#foo_string_3").unwrap(),
        //     "foo_string_3"
        // );
        // assert_eq!(
        //     u.exp_attr_names.get("#foo_string_4").unwrap(),
        //     "foo_string_4"
        // );
        // assert_eq!(
        //     u.exp_attr_values.get(":foo_string_3_u0").unwrap(),
        //     &AttributeValue::S("ccc".to_string())
        // );
        // assert_eq!(
        //     u.exp_attr_values.get(":foo_string_4_u0").unwrap(),
        //     &AttributeValue::S("ddd".to_string())
        // );
    }
}

// match self
//     .schema
//     .attributes
//     .get(update_key.as_str())
//     .ok_or(DeezError::UnknownAttribute(update_key.to_string()))?
// {
//     DynamoType::DynamoString => {
//         self.exp_attr_values.insert(
//             format!(":{}", exp_value_var),
//             AttributeValue::S(update_value.to_string()),
//         );
//     }
//     _ => panic!(), // todo: Err()
// }
