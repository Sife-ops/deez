use crate::deez::{DeezEntity, DeezError, DeezResult};
use crate::types::schema::{
    composed_key, get_composed_index, composed_index, DynamoType, Index, IndexKey, IndexKeysComposed, Schema,
};
use aws_sdk_dynamodb::{
    operation::update_item::builders::UpdateItemFluentBuilder, types::AttributeValue,
};
use std::collections::HashMap;

impl super::Deez {
    // todo: patch
    pub fn update(&self, entity: &impl DeezEntity) -> DeezResult<DeezUpdateBuilder> {
        let i = get_composed_index!(entity, Index::Primary);
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

            exp_attr_names: HashMap::new(),
            exp_attr_values: HashMap::new(),
            sets: Vec::new(),
            adds: Vec::new(),
            subtracts: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct DeezUpdateBuilder {
    pub builder: UpdateItemFluentBuilder,
    pub schema: Schema,
    pub av_map: HashMap<String, AttributeValue>,

    pub exp_attr_names: HashMap<String, String>,
    pub exp_attr_values: HashMap<String, AttributeValue>,
    pub sets: Vec<String>,
    pub adds: Vec<String>,
    pub subtracts: Vec<String>,
}

macro_rules! unique_exp_value_var {
    ($self: ident, $update_key: ident) => {{
        let matches = $self
            .exp_attr_values
            .iter()
            .filter(|(x, _)| x.starts_with(&format!(":{}", $update_key)))
            .collect::<HashMap<&String, &AttributeValue>>();
        format!("{}_u{}", $update_key, matches.len())
    }};
}

macro_rules! add_exp {
    ($self: ident, $aaa: ident, $bbb: ident, $ccc: expr, $ddd: ident, $eee: expr) => {
        let exp_value_var = unique_exp_value_var!($self, $aaa);
        $self.$bbb.push(format!($ccc, $aaa, exp_value_var));
        $self
            .exp_attr_names
            .insert(format!("#{}", $aaa), $aaa.clone());
        $self
            .exp_attr_values
            .insert(format!(":{}", exp_value_var), AttributeValue::$ddd($eee));
    };
}

macro_rules! sync_key {
    ($self: ident, $index_key: expr, $update_key: ident) => {
        for composite in $index_key.composite() {
            if composite == $update_key {
                let field = $index_key.field();
                let composed = composed_key!($index_key, $self.schema, $self.av_map);
                add_exp!($self, field, sets, "#{} = :{}", S, composed);
            }
        }
    };
}

impl DeezUpdateBuilder {
    // todo: string-only composites?
    pub fn set_string(mut self, map: HashMap<String, String>) -> DeezResult<DeezUpdateBuilder> {
        for (update_key, update_value) in map.iter() {
            *self.av_map.get_mut(update_key).unwrap() = AttributeValue::S(update_value.to_string());
        }
        for (update_key, update_value) in map.iter() {
            add_exp!(
                self,
                update_key,
                sets,
                "#{} = :{}",
                S,
                update_value.to_string()
            );
            for (_, index_keys) in self.schema.global_secondary_indexes.iter() {
                sync_key!(self, index_keys.partition_key, update_key);
                sync_key!(self, index_keys.sort_key, update_key);
            }
        }
        Ok(self)
    }

    pub fn set_number(mut self, map: HashMap<String, f64>) -> DeezResult<DeezUpdateBuilder> {
        for (update_key, update_value) in map.iter() {
            add_exp!(
                self,
                update_key,
                sets,
                "#{} = :{}",
                N,
                update_value.to_string()
            );
        }
        Ok(self)
    }

    pub fn set_bool(mut self, map: HashMap<String, bool>) -> DeezResult<DeezUpdateBuilder> {
        for (update_key, update_value) in map.iter() {
            add_exp!(self, update_key, sets, "#{} = :{}", Bool, *update_value);
        }
        Ok(self)
    }

    pub fn add(mut self, map: HashMap<String, f64>) -> DeezResult<DeezUpdateBuilder> {
        for (update_key, update_value) in map.iter() {
            add_exp!(
                self,
                update_key,
                adds,
                "#{} :{}",
                N,
                update_value.to_string()
            );
        }
        Ok(self)
    }

    pub fn subtract(mut self, map: HashMap<String, f64>) -> DeezResult<DeezUpdateBuilder> {
        for (update_key, update_value) in map.iter() {
            let exp_value_var = unique_exp_value_var!(self, update_key);
            self.subtracts.push(format!(
                "#{} = #{} - :{}",
                update_key, update_key, exp_value_var
            ));
            self.exp_attr_names
                .insert(format!("#{}", update_key), update_key.clone());
            self.exp_attr_values.insert(
                format!(":{}", exp_value_var),
                AttributeValue::N(update_value.to_string()),
            );
        }
        Ok(self)
    }

    pub fn build(self) -> UpdateItemFluentBuilder {
        let mut exp = String::new();

        if self.sets.len() > 0 {
            for (i, e) in self.sets.iter().enumerate() {
                match i {
                    0 => exp.push_str(&format!("SET {}", e)),
                    _ => exp.push_str(&format!(", {}", e)),
                }
            }
        }

        if self.subtracts.len() > 0 {
            for (i, e) in self.subtracts.iter().enumerate() {
                match i {
                    0 => {
                        if exp.len() < 1 {
                            exp.push_str(&format!("SET {}", e));
                        } else {
                            exp.push_str(&format!(", {}", e));
                        }
                    }
                    _ => exp.push_str(&format!(", {}", e)),
                }
            }
        }

        if self.adds.len() > 0 {
            match exp.len() {
                0 => exp.push_str("ADD"),
                _ => exp.push_str(" ADD"),
            }
            for (i, e) in self.adds.iter().enumerate() {
                match i {
                    0 => exp.push_str(&format!(" {}", e)),
                    _ => exp.push_str(&format!(", {}", e)),
                }
            }
        }

        println!("{}", exp);
        println!("{:#?}", self.exp_attr_names);
        println!("{:#?}", self.exp_attr_values);

        self.builder
            .update_expression(exp)
            .set_expression_attribute_names(Some(self.exp_attr_names))
            .set_expression_attribute_values(Some(self.exp_attr_values))
    }
}

#[cfg(test)]
mod tests {
    use crate::{mocks::mocks::*, DeezResult};
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    #[tokio::test]
    async fn update_builder() -> DeezResult<()> {
        let d = make_mock_deez().await;

        let u = d
            .update(&Foo {
                foo_string_1: "aaa".to_string(),
                ..Default::default()
            })?
            .set_string(HashMap::from([
                ("foo_string_2".to_string(), "rofl".to_string()),
                ("foo_string_6".to_string(), "lol".to_string()),
            ]))?
            .set_string(HashMap::from([
                // ("foo_string_6".to_string(), "lmao".to_string()),
                ("foo_string_2".to_string(), "lmao".to_string()),
            ]))?
            .add(HashMap::from([
                // ("foo_string_2".to_string(), "lmao".to_string()),
                ("foo_isize".to_string(), 3.0),
            ]))?
            .add(HashMap::from([
                // ("foo_string_2".to_string(), "lmao".to_string()),
                ("foo_isize".to_string(), 2.0),
            ]))?
            .subtract(HashMap::from([
                // ("foo_string_2".to_string(), "lmao".to_string()),
                ("foo_isize".to_string(), 1.0),
            ]))?
            .build();

        // println!("{:#?}", u);
        // println!("{:#?}", u.exp);
        // println!("{:#?}", u.exp_attr_names);
        // println!("{:#?}", u.exp_attr_values);

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

        Ok(())
    }
}
