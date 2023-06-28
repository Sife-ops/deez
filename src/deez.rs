use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug)]
pub struct IndexKeys {
    pub hash: IndexKey,
    pub range: IndexKey,
}

#[derive(Debug, Default)]
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
    fn all_types() {
        let a = Buss {
            string_opt: Some("yeah".to_string()),
            ..Default::default()
        };

        let b: &HashMap<String, AttributeValue> = &a.into();
        println!("{:#?}", b);

        let c: Buss = b.into();
        println!("{:#?}", c);

        // todo: assert
    }

    #[test]
    fn index_names() {
        assert_eq!(Task::gsi1_name(), "task_gsi1");
        assert_eq!(Task::gsi2_name(), "task_gsi2");
    }

    #[test]
    fn to_from() {
        {
            let a = Task {
                task_id: Some("1a2b3c4d".to_string()),
                project: Some("foo_project".to_string()),
                employee: Some("e42069".to_string()),
                description: "nothin' but chillin' 20's".to_string(),
                some_metadata: "baz".to_string(),
            };

            let b: &HashMap<String, AttributeValue> = &a.into();
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

            let c: Task = b.into();
            println!("{:#?}", c);

            assert_eq!(c.task_id, Some("1a2b3c4d".to_string()));
            assert_eq!(c.project, Some("foo_project".to_string()));
            assert_eq!(c.employee, Some("e42069".to_string()));
            assert_eq!(c.description, "nothin' but chillin' 20's".to_string());
            assert_eq!(c.some_metadata, "it's true".to_string());
        }
    }

    #[test]
    fn partial_keys() {
        let mut task = Task {
            task_id: Some("1a2b3c4d".to_string()),
            project: None,
            employee: None,
            description: "nothin' but chillin' 20's".to_string(),
            some_metadata: "baz".to_string(),
            ..Default::default()
        };

        {
            let map: HashMap<String, AttributeValue> = task.clone().into();
            // println!("{:#?}", map);
            assert_eq!(map["sk"], AttributeValue::S("$Task".to_string()));
        }

        {
            task.employee = Some("e42069".to_string());
            let map: HashMap<String, AttributeValue> = task.clone().into();
            assert_eq!(
                map["sk"],
                AttributeValue::S("$Task#employee_e42069".to_string())
            );
        }
    }
}
