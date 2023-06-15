use std::collections::HashMap;

#[derive(Debug)]
pub struct Schema {
    pub table: &'static str,
    pub service: &'static str,
    pub entity: &'static str,
    pub primary_index: IndexKeys,
    pub global_secondary_indexes: HashMap<Index, IndexKeys>,
    pub attributes: HashMap<&'static str, DynamoType>,
}

#[derive(Debug)]
pub struct IndexKeys {
    pub partition_key: IndexKey,
    pub sort_key: IndexKey,
}

#[derive(Debug)]
pub struct IndexKeysComposed {
    pub partition_key: (String, String),
    pub sort_key: (String, String),
}

#[derive(Debug)]
pub enum DynamoType {
    DynamoString,
    DynamoNumber,
    DynamoBool,
}

#[derive(Debug)]
pub enum IndexKey {
    Partition(Key),
    Sort(Key),
}

#[derive(Debug)]
pub struct Key {
    pub field: &'static str,
    pub composite: Vec<&'static str>,
}

impl IndexKey {
    pub fn field(&self) -> String {
        match self {
            IndexKey::Partition(k) => k.field.to_string(),
            IndexKey::Sort(k) => k.field.to_string(),
        }
    }

    pub fn composite(&self) -> Vec<&str> {
        match self {
            IndexKey::Partition(k) => k.composite.clone(),
            IndexKey::Sort(k) => k.composite.clone(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Index {
    Primary,
    Gsi1(&'static str),
    Gsi2(&'static str),
    Gsi3(&'static str),
    Gsi4(&'static str),
    Gsi5(&'static str),
    Gsi6(&'static str),
    Gsi7(&'static str),
    Gsi8(&'static str),
    Gsi9(&'static str),
    Gsi10(&'static str),
    Gsi11(&'static str),
    Gsi12(&'static str),
    Gsi13(&'static str),
    Gsi14(&'static str),
    Gsi15(&'static str),
    Gsi16(&'static str),
    Gsi17(&'static str),
    Gsi18(&'static str),
    Gsi19(&'static str),
    Gsi20(&'static str),
}

impl std::fmt::Display for Index {
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

macro_rules! composed_key {
    ($index_key: expr, $schema: expr, $av_map: expr) => {{
        let mut a = String::new();

        let v;
        match &$index_key {
            IndexKey::Partition(k) => {
                v = k.composite.clone();
                a.push_str(&format!("${}#{}", $schema.service, $schema.entity));
            }
            IndexKey::Sort(k) => {
                v = k.composite.clone();
                a.push_str(&format!("${}", $schema.entity));
            }
        }

        for b in v {
            let d = $av_map
                .get(b)
                .ok_or(DeezError::UnknownAttribute(b.to_string()))?;
            let c = $schema
                .attributes
                .get(b)
                .ok_or(DeezError::UnknownAttribute(b.to_string()))?;
            match c {
                DynamoType::DynamoString => a.push_str(&format!("#{}_{}", b, d.as_s()?)),
                DynamoType::DynamoNumber => a.push_str(&format!("#{}_{}", b, d.as_n()?)),
                _ => return Err(DeezError::InvalidComposite),
            }
        }

        a
    }};
}
pub(crate) use composed_key;

macro_rules! composed_index {
    ($index_keys: expr, $schema: ident, $av_map: ident) => {{
        IndexKeysComposed {
            partition_key: (
                $index_keys.partition_key.field(),
                composed_key!($index_keys.partition_key, $schema, $av_map),
            ),
            sort_key: (
                $index_keys.sort_key.field(),
                composed_key!($index_keys.sort_key, $schema, $av_map),
            ),
        }
    }};
}
pub(crate) use composed_index;

macro_rules! get_composed_index {
    ($entity: ident, $index: expr) => {{
        let av_map = $entity.to_av_map()?;
        let schema = $entity.schema();
        match $index {
            Index::Primary => composed_index!(schema.primary_index, schema, av_map),
            _ => composed_index!(
                schema
                    .global_secondary_indexes
                    .get(&$index)
                    .ok_or(DeezError::UnknownSchemaIndex($index.to_string()))?,
                schema,
                av_map
            ),
        }
    }};
}
pub(crate) use get_composed_index;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deez::DeezSchema, mocks::mocks::*, DeezEntity, DeezResult};

    #[test]
    fn macros() -> DeezResult<()> {
        let a = Foo {
            foo_string_1: "aaa".to_string(),
            foo_string_2: "bbb".to_string(),
            foo_string_3: "ccc".to_string(),
            foo_string_4: "ddd".to_string(),
            foo_string_5: "eee".to_string(),
            foo_string_6: "fff".to_string(),
            foo_bool: true,
            ..Default::default()
        };

        let b = a.to_av_map()?;
        let c = a.schema();

        let d = c.primary_index;
        // let e = composed_index!(d, c, b);

        // let c = b.primary_index.partition_key;
        // println!("{:#?}", e);
        // let e = composed_key!(c, b, d);
        // println!("{}", e);

        Ok(())
    }
}
