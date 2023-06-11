use crate::types::schema::{DynamoType, Schema};
use crate::DeezError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

pub type DeezResult<T> = Result<T, DeezError>;

pub struct Deez {
    client: Client,
}

impl Deez {
    pub fn new(c: Client) -> Deez {
        Deez { client: c }
    }
}

// mod create;

// mod batch_write;

// mod delete;

// mod query;

// mod update;

// todo: scan
// todo: batch get
// todo: patch

// todo: DeezEntityPartial, to_av_map

pub trait DeezSchema {
    fn schema(&self) -> Schema;
}

pub trait DeezEntity: DeezSchema + bevy_reflect::Struct {
    // todo: impl from?
    fn from_av_map(m: &HashMap<String, AttributeValue>) -> DeezResult<Self>
    where
        Self: Sized;

    fn to_av_map(&self) -> HashMap<String, AttributeValue> {
        let mut av_map: HashMap<String, AttributeValue> = HashMap::new();

        let schema = self.schema();

        for (i, value) in self.iter_fields().enumerate() {
            // todo: rename
            let field_name = self.name_at(i).unwrap();
            let attribute = schema.attributes.get(field_name).unwrap();

            // todo: list types
            // todo: optional types
            match attribute.dynamo_type {
                DynamoType::DynamoString => {
                    av_map.insert(
                        field_name.to_string(),
                        AttributeValue::S(value.downcast_ref::<String>().unwrap().to_string()),
                    );
                }
                DynamoType::DynamoBool => {
                    av_map.insert(
                        field_name.to_string(),
                        AttributeValue::Bool(value.downcast_ref::<bool>().unwrap().clone()),
                    );
                }
                DynamoType::DynamoNumber => {
                    // todo: other num types
                    av_map.insert(
                        field_name.to_string(),
                        AttributeValue::N(value.downcast_ref::<u8>().unwrap().to_string()),
                    );
                }
            }
        }

        av_map
    }

    fn to_av_map_with_keys(&self) -> HashMap<String, AttributeValue>
    where
        Self: Sized,
    {
        let mut m = self.to_av_map();
        let s = self.schema();

        let b = s.primary_index.composed_index(self);
        {
            let (c, d) = b.partition_key;
            m.insert(c, AttributeValue::S(d));
        }
        {
            let (c, d) = b.sort_key;
            m.insert(c, AttributeValue::S(d));
        }

        for (_, c) in s.global_secondary_indexes {
            let d = c.composed_index(self);
            {
                let (e, f) = d.partition_key;
                m.insert(e, AttributeValue::S(f));
            }
            {
                let (e, f) = d.sort_key;
                m.insert(e, AttributeValue::S(f));
            }
        }

        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::mocks::*;

    #[tokio::test]
    async fn to_from() {
        let a = Foo {
            foo_string_1: "aaa".to_string(),
            foo_string_2: "bbb".to_string(),
            foo_string_3: "ccc".to_string(),
            foo_string_4: "ddd".to_string(),
            foo_usize: 33,
            foo_bool: true,
        };

        let b = a.to_av_map_with_keys();

        assert_eq!(
            b["pk"],
            AttributeValue::S("$fooservice#fooentity#foo_string_1_aaa".to_string())
        );
        assert_eq!(b["sk"], AttributeValue::S("$fooentity".to_string()));
        assert_eq!(
            b["gsi1pk"],
            AttributeValue::S("$fooservice#fooentity#foo_string_2_bbb".to_string())
        );
        assert_eq!(
            b["gsi1sk"],
            AttributeValue::S("$fooentity#foo_string_1_aaa".to_string())
        );

        // let c= Foo::from_av_map(&b).unwrap();
        let c= Foo::from(&b);

        // println!("{:#?}", c);

        assert_eq!(c.foo_string_1, "aaa".to_string());
        assert_eq!(c.foo_string_2, "bbb".to_string());
        assert_eq!(c.foo_string_3, "ccc".to_string());
        assert_eq!(c.foo_string_4, "ddd".to_string());
        assert_eq!(c.foo_usize, 33);
        assert_eq!(c.foo_bool, true);
    }
}
