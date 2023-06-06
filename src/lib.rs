mod error;

use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::operation::update_item::builders::UpdateItemFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
pub use deez_derive::DeezEntity;
use error::DeezError;
use std::collections::HashMap;

pub struct Deez {
    client: Client,
}

impl Deez {
    pub fn new(c: Client) -> Self {
        Deez { client: c }
    }

    pub fn put(&self, entity: &impl DeezEntity) -> Result<PutItemFluentBuilder, DeezError> {
        Ok(self
            .client
            .put_item()
            .table_name(entity.meta().table)
            .set_item(Some(entity.to_av_map_keys()?)))
    }

    pub fn query(
        &self,
        index: Index,
        entity: &impl DeezEntity,
    ) -> Result<QueryFluentBuilder, DeezError> {
        let index_keys = entity.index_keys();
        let i = index_keys
            .get(&index)
            .ok_or(DeezError::MapKey(index.to_string()))?;
        let pkf = i.partition_key.field.clone();
        let skf = i.sort_key.field.clone();
        // todo: verify the index composites exist in av
        let av = entity.to_av_map_keys()?;

        let mut request = self
            .client
            .query()
            .table_name(entity.meta().table)
            .key_condition_expression(format!("#{pkf} = :{pkf} and begins_with(#{skf}, :{skf})"))
            .expression_attribute_names(format!("#{pkf}"), pkf)
            .expression_attribute_names(format!("#{skf}"), skf)
            .expression_attribute_values(format!(":{pkf}"), av.get(pkf).unwrap().clone())
            .expression_attribute_values(format!(":{skf}"), av.get(skf).unwrap().clone());

        if index != Index::Primary {
            request = request.index_name(index.to_string());
        }

        Ok(request)
    }

    pub fn update(&self, entity: &impl DeezEntity) -> Result<UpdateItemFluentBuilder, DeezError> {
        let av_map = entity.to_av_map_keys()?;

        let index_keys = entity.index_keys();

        let primary_index = index_keys
            .get(&Index::Primary)
            .ok_or(DeezError::UnknownKey(Index::Primary.to_string()))?;

        let pk = primary_index.partition_key.field;
        let sk = primary_index.sort_key.field;

        let mut update_expression = String::from("SET");
        let av_map_attr = entity.to_av_map();
        av_map_attr.iter().enumerate().for_each(|(i, v)| match i {
            0 => update_expression.push_str(&format!(" #{} = :{}", v.0, v.0)),
            _ => update_expression.push_str(&format!(", #{} = :{}", v.0, v.0)),
        });

        let mut request = self
            .client
            .update_item()
            .table_name(entity.meta().table)
            .key(
                pk,
                av_map
                    .get(pk)
                    .ok_or(DeezError::MapKey(pk.to_string()))?
                    .clone(),
            )
            .key(
                sk,
                av_map
                    .get(sk)
                    .ok_or(DeezError::MapKey(sk.to_string()))?
                    .clone(),
            )
            .update_expression(update_expression);

        for (k, _) in av_map_attr.iter() {
            request = request.expression_attribute_names(format!("#{}", k), k);
        }
        for (k, v) in av_map_attr.iter() {
            request = request.expression_attribute_values(format!(":{}", k), v.clone());
        }

        Ok(request)
    }
}

#[derive(Debug)]
pub struct Meta<'a> {
    pub table: &'a str,
    pub service: &'a str,
    pub entity: &'a str,
}

#[derive(Debug)]
pub struct IndexKeys<'a> {
    pub partition_key: Key<'a>,
    pub sort_key: Key<'a>,
}

#[derive(Debug)]
pub struct Key<'a> {
    pub field: &'a str,
    pub composite: Vec<String>, // todo: Vec<&'a str>?
}

impl Key<'_> {
    fn _join_composite(
        &self,
        attrs: &HashMap<String, AttributeValue>,
    ) -> Result<String, DeezError> {
        let mut j = String::new();
        for c in self.composite.iter() {
            let a = attrs.get(c).ok_or(DeezError::MapKey(c.to_string()))?;
            let s = match a {
                AttributeValue::S(b) => b.to_string(),
                AttributeValue::N(b) => b.to_string(),
                AttributeValue::Bool(b) => b.to_string(),
                _ => return Err(DeezError::InvalidComposite(c.to_string())),
            };
            j.push_str(&format!("#{}_{}", c, s));
        }
        Ok(j)
    }
}

pub trait DeezMeta {
    fn meta(&self) -> Meta;
    fn index_keys(&self) -> HashMap<Index, IndexKeys>;
    fn generated() -> Self;
}

pub trait DeezEntity: DeezMeta {
    fn to_av_map(&self) -> HashMap<String, AttributeValue>;
    fn to_av_map_keys(&self) -> Result<HashMap<String, AttributeValue>, DeezError>;
    fn from_av_map(m: HashMap<String, AttributeValue>) -> Result<Self, DeezError>
    where
        Self: Sized;
}

#[derive(Eq, Hash, PartialEq)]
pub enum Index<'a> {
    Primary,
    Gsi1(&'a str),
    Gsi2(&'a str),
    Gsi3(&'a str),
    Gsi4(&'a str),
    Gsi5(&'a str),
    Gsi6(&'a str),
    Gsi7(&'a str),
    Gsi8(&'a str),
    Gsi9(&'a str),
    Gsi10(&'a str),
    Gsi11(&'a str),
    Gsi12(&'a str),
    Gsi13(&'a str),
    Gsi14(&'a str),
    Gsi15(&'a str),
    Gsi16(&'a str),
    Gsi17(&'a str),
    Gsi18(&'a str),
    Gsi19(&'a str),
    Gsi20(&'a str),
}

impl std::fmt::Display for Index<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Primary => write!(f, "primary"),
            Index::Gsi1(x) => write!(f, "{}", x),
            Index::Gsi2(x) => write!(f, "{}", x),
            Index::Gsi3(x) => write!(f, "{}", x),
            Index::Gsi4(x) => write!(f, "{}", x),
            Index::Gsi5(x) => write!(f, "{}", x),
            Index::Gsi6(x) => write!(f, "{}", x),
            Index::Gsi7(x) => write!(f, "{}", x),
            Index::Gsi8(x) => write!(f, "{}", x),
            Index::Gsi9(x) => write!(f, "{}", x),
            Index::Gsi10(x) => write!(f, "{}", x),
            Index::Gsi11(x) => write!(f, "{}", x),
            Index::Gsi12(x) => write!(f, "{}", x),
            Index::Gsi13(x) => write!(f, "{}", x),
            Index::Gsi14(x) => write!(f, "{}", x),
            Index::Gsi15(x) => write!(f, "{}", x),
            Index::Gsi16(x) => write!(f, "{}", x),
            Index::Gsi17(x) => write!(f, "{}", x),
            Index::Gsi18(x) => write!(f, "{}", x),
            Index::Gsi19(x) => write!(f, "{}", x),
            Index::Gsi20(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod int_tests {
    use super::*;

    // todo: initialize the table

    async fn make_test_client() -> Deez {
        let config = aws_config::from_env()
            .endpoint_url("http://localhost:8000")
            .region("us-east-1")
            .load()
            .await;
        Deez::new(Client::new(&config))
    }

    ////////////////////////////////////////////////////////////////////////////
    /// move to a higher scope...
    const PRIMARY: Index = Index::Primary;

    #[derive(DeezEntity, Debug, Default)]
    pub struct Foo {
        pub foo_string: String,
        pub foo_usize: u8,
        pub foo_bool: bool,
    }

    impl DeezMeta for Foo {
        fn meta(&self) -> Meta {
            Meta {
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
            }
        }
        fn index_keys(&self) -> HashMap<Index, IndexKeys> {
            let mut m = HashMap::new();
            m.insert(
                PRIMARY,
                IndexKeys {
                    partition_key: Key {
                        field: "pk",
                        composite: vec!["foo_string".to_string()],
                    },
                    sort_key: Key {
                        field: "sk",
                        composite: vec![],
                    },
                },
            );
            m
        }
        fn generated() -> Self {
            Foo {
                ..Default::default()
            }
        }
    }
    ////////////////////////////////////////////////////////////////////////////

    #[cfg(test)]
    #[tokio::test]
    async fn lmao() {
        println!("ye");
        let d = make_test_client().await;
        let r = d
            .put(&Foo {
                foo_string: "asdf".to_string(),
                foo_usize: 69,
                foo_bool: false,
            })
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(1, 1);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    const PRIMARY: Index = Index::Primary;

    #[derive(DeezEntity, Debug, Default)]
    pub struct Foo {
        pub foo_string: String,
        #[deez(rename = "fooz")]
        // #[deez(skip)]
        pub foo_usize: u8,
        pub foo_bool: bool,
        #[deez(skip)]
        pub foo_skip: String,
        // todo: other types
    }

    impl DeezMeta for Foo {
        fn meta(&self) -> Meta {
            Meta {
                table: "footable",
                service: "fooservice",
                entity: "fooentity",
            }
        }
        fn index_keys(&self) -> HashMap<Index, IndexKeys> {
            let mut m = HashMap::new();
            m.insert(
                PRIMARY,
                IndexKeys {
                    partition_key: Key {
                        field: "pk",
                        composite: vec!["foo_string".to_string()],
                    },
                    sort_key: Key {
                        field: "sk",
                        composite: vec!["fooz".to_string()],
                    },
                },
            );
            m
        }
        fn generated() -> Self {
            Foo {
                ..Default::default()
            }
        }
    }

    // #[test]
    // fn t1() {
    //     let a = Foo {
    //         foo_string: format!("bar"),
    //         foo_usize: 3,
    //         foo_bool: true,
    //         foo_skip: format!("plz skip"),
    //     };

    //     ////////////////////////////////////////////////////////////////////////
    //     let b = a.to_av_map_keys().unwrap();
    //     println!("{:#?}", b);

    //     assert_eq!(
    //         b.get("foo_string").unwrap().as_s().unwrap().to_string(),
    //         "bar".to_string()
    //     );
    //     assert_eq!(b.get("fooz").unwrap().as_n().unwrap().to_string(), "3");

    //     ////////////////////////////////////////////////////////////////////////
    //     let c = Foo::from_av_map(b);
    //     println!("{:#?}", c);
    // }
}
