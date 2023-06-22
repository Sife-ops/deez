use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug)]
pub struct IndexKeys {
    pub hash: IndexKey,
    pub range: IndexKey,
}

#[derive(Debug)]
pub struct IndexKey {
    pub field: String,
    pub composite: String,
}

impl IndexKey {
    pub fn field(&self) -> String {
        self.field.clone()
    }
    pub fn av(&self) -> AttributeValue {
        AttributeValue::S(self.composite.clone())
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    Hash,
    Range,
}

#[cfg(test)]
mod tests {
    use crate::mocks::mocks::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    #[test]
    fn index_names() {
        assert_eq!(Foo::gsi1_name(), "foo_gsi1");
        assert_eq!(Foo::gsi2_name(), "foo_gsi2");
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
            // println!("{:#?}", b);

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
