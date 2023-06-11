#[derive(Debug)]
pub struct IndexKeys<T> {
    pub hash: IndexKey<T>,
    pub range: IndexKey<T>,
}

#[derive(Debug)]
pub struct IndexKey<T> {
    pub field: String,
    pub composite: T,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    Hash,
    Range,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Index {
    Primary,
    Gsi1,
    Gsi2,
    Gsi3,
    Gsi4,
    Gsi5,
    Gsi6,
    Gsi7,
    Gsi8,
    Gsi9,
    Gsi10,
    Gsi11,
    Gsi12,
    Gsi13,
    Gsi14,
    Gsi15,
    Gsi16,
    Gsi17,
    Gsi18,
    Gsi19,
    Gsi20,
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Primary => write!(f, "Primary"),
            Index::Gsi1 => write!(f, "Gsi1"),
            Index::Gsi2 => write!(f, "Gsi2"),
            Index::Gsi3 => write!(f, "Gsi3"),
            Index::Gsi4 => write!(f, "Gsi4"),
            Index::Gsi5 => write!(f, "Gsi5"),
            Index::Gsi6 => write!(f, "Gsi6"),
            Index::Gsi7 => write!(f, "Gsi7"),
            Index::Gsi8 => write!(f, "Gsi8"),
            Index::Gsi9 => write!(f, "Gsi9"),
            Index::Gsi10 => write!(f, "Gsi10"),
            Index::Gsi11 => write!(f, "Gsi11"),
            Index::Gsi12 => write!(f, "Gsi12"),
            Index::Gsi13 => write!(f, "Gsi13"),
            Index::Gsi14 => write!(f, "Gsi14"),
            Index::Gsi15 => write!(f, "Gsi15"),
            Index::Gsi16 => write!(f, "Gsi16"),
            Index::Gsi17 => write!(f, "Gsi17"),
            Index::Gsi18 => write!(f, "Gsi18"),
            Index::Gsi19 => write!(f, "Gsi19"),
            Index::Gsi20 => write!(f, "Gsi20"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::mocks::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    #[test]
    fn index_names() {
        assert_eq!(Foo::index_name(Index::Gsi1), "gsi1");
        assert_eq!(Foo::index_name(Index::Gsi2), "gsi2");
    }

    #[test]
    fn to_from() {
        {
            let a = Foo {
                foo_string_1: "aaa".to_string(),
                foo_string_2: "bbb".to_string(),
                foo_string_3: "ccc".to_string(),
                foo_string_4: "ddd".to_string(),
                foo_string_5: "eee".to_string(),
                foo_string_6: "fff".to_string(),
                ..Default::default()
            };

            let b: HashMap<String, AttributeValue> = a.into();
            // println!("{:#?}", b);

            assert_eq!(
                b["pk"],
                AttributeValue::S("$foo_service#foo_entity#foo_string_1_aaa".to_string())
            );
            assert_eq!(
                b["sk"],
                AttributeValue::S("$foo_entity#foo_string_2_bbb#foo_string_3_ccc".to_string())
            );
            assert_eq!(
                b["gsi1pk"],
                AttributeValue::S("$foo_service#foo_entity#foo_string_4_ddd".to_string())
            );
            assert_eq!(
                b["gsi1sk"],
                AttributeValue::S("$foo_entity#foo_num1_69".to_string())
            );
            assert_eq!(
                b["gsi2pk"],
                AttributeValue::S("$foo_service#foo_entity#foo_string_5_eee".to_string())
            );
            assert_eq!(b["gsi2sk"], AttributeValue::S("$foo_entity".to_string()));
        }

        {
            let a = Task {
                task_id: "1a2b3c4d".to_string(),
                project: "foo_project".to_string(),
                employee: "e42069".to_string(),
                description: "nothin' but chillin' 20's".to_string(),
                some_metadata: "baz".to_string(),
            };

            let b: HashMap<String, AttributeValue> = a.into();
            println!("{:#?}", b);

            assert_eq!(
                b["pk"],
                AttributeValue::S("$TaskService#Task#task_id_1a2b3c4d".to_string())
            );
            assert_eq!(
                b["sk"],
                AttributeValue::S("$Task#employee_e42069#project_foo_project".to_string())
            );
            assert_eq!(
                b["gsi1pk"],
                AttributeValue::S("$TaskService#Task#project_foo_project".to_string())
            );
            assert_eq!(
                b["gsi1sk"],
                AttributeValue::S("$Task#employee_e42069#task_id_1a2b3c4d".to_string())
            );
            assert_eq!(
                b["gsi2pk"],
                AttributeValue::S("$TaskService#Task#employee_e42069".to_string())
            );
            assert_eq!(
                b["gsi2sk"],
                AttributeValue::S("$Task#project_foo_project#task_id_1a2b3c4d".to_string())
            );
        }
    }
}
