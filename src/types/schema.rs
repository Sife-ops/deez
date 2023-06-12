use crate::{deez::DeezEntity, DeezError, DeezResult};
use bevy_reflect::GetField;
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
    pub partition_key: Key,
    pub sort_key: Key,
}

#[derive(Debug)]
pub struct IndexKeysComposed {
    pub partition_key: (String, String),
    pub sort_key: (String, String),
}

impl IndexKeys {
    pub fn composed_index(&self, e: &impl DeezEntity) -> DeezResult<IndexKeysComposed> {
        let a = e.schema();
        Ok(IndexKeysComposed {
            partition_key: (
                self.partition_key.field.to_string(),
                format!(
                    "${}#{}{}",
                    a.service,
                    a.entity,
                    self.partition_key.composed_key(e)?
                ),
            ),
            sort_key: (
                self.sort_key.field.to_string(),
                format!("${}{}", a.entity, self.sort_key.composed_key(e)?),
            ),
        })
    }
}

#[derive(Debug)]
pub enum RustType {
    Usize,
    Isize,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

#[derive(Debug)]
pub enum DynamoType {
    DynamoString,
    DynamoNumber(RustType),
    DynamoBool,
}

#[derive(Debug)]
pub struct Key {
    pub field: &'static str,
    pub composite: Vec<&'static str>,
}

impl Key {
    pub fn composed_key(&self, e: &impl DeezEntity) -> DeezResult<String> {
        let mut a = String::new();
        let s = e.schema();

        for b in self.composite.iter() {
            let d = s
                .attributes
                .get(b)
                .ok_or(DeezError::UnknownAttribute(b.to_string()))?;

            let x: String;
            match d {
                DynamoType::DynamoBool => return Err(DeezError::InvalidComposite),
                DynamoType::DynamoString => {
                    x = e
                        .get_field::<String>(b)
                        .ok_or(DeezError::UnknownStructField(b.to_string()))?
                        .to_string();
                }
                DynamoType::DynamoNumber(rt) => match rt {
                    RustType::Usize => {
                        x = e
                            .get_field::<usize>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::Isize => {
                        x = e
                            .get_field::<isize>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::U8 => {
                        x = e
                            .get_field::<u8>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::I8 => {
                        x = e
                            .get_field::<i8>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::U16 => {
                        x = e
                            .get_field::<u16>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::I16 => {
                        x = e
                            .get_field::<i16>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::U32 => {
                        x = e
                            .get_field::<u32>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::I32 => {
                        x = e
                            .get_field::<i32>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::U64 => {
                        x = e
                            .get_field::<u64>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                    RustType::I64 => {
                        x = e
                            .get_field::<i64>(b)
                            .ok_or(DeezError::UnknownStructField(b.to_string()))?
                            .to_string();
                    }
                },
            }

            a.push_str(&format!("#{}_{}", b, x));
        }

        Ok(a)
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
