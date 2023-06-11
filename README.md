A DynamoDB abstraction for Rust
==========================================================
[![Crates.io](https://img.shields.io/crates/v/deez.svg)](https://crates.io/crates/deez)

Deez is a DynamoDB abstraction for implementing Single Table Design easily,
inspired by [ElectroDB](https://github.com/tywalch/electrodb).

## Getting Started

Define a schema for your entities using the `Deez` procedural macro. Doing so
will derive the `From` conversion traits for your structs and the
`HashMap<String, AttributeValue>` type used by the `aws_sdk_dynamodb` library,
with some additional features for faciliting Single Table Design.

```rust
use aws_sdk_dynamodb::types::AttributeValue;
use deez::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Deez)]
#[deez_schema(table = "TaskTable", service = "TaskService", entity = "Task")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2_name = "gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Task {
    #[deez_primary(key = "hash")]
    #[deez_gsi1(key = "range", position = 1)]
    #[deez_gsi2(key = "range", position = 1)]
    pub task_id: String,
    #[deez_primary(key = "range", position = 1)]
    #[deez_gsi1(key = "hash")]
    #[deez_gsi2(key = "range")]
    pub project: String,
    #[deez_primary(key = "range")]
    #[deez_gsi1(key = "range")]
    #[deez_gsi2(key = "hash")]
    pub employee: String,
    pub description: String,
    #[deez_ignore(ignore)]
    pub some_metadata: String,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            task_id: Uuid::new_v4().to_string(),
            project: "".to_string(),
            employee: "".to_string(),
            description: "".to_string(),
            some_metadata: "".to_string(),
        }
    }
}
```

Now you can convert your struct to a `HashMap` that you can pass directly to the
dynamodb client.

```rust
let task = Task {
    task_id: "1a2b3c4d".to_string(),
    project: "foo_project".to_string(),
    employee: "e42069".to_string(),
    description: "nothin' but chillin' 20's".to_string(),
    some_metadata: "baz".to_string(),
};

let map: HashMap<String, AttributeValue> = task.into();
println!("{:#?}", map);

// output:
// {
//     "pk": S("$TaskService#Task#task_id_1a2b3c4d"),             <- keys generated based on schema
//     "sk": S("$Task#employee_e42069#project_foo_project"),      <-
//     "gsi1pk": S("$TaskService#Task#project_foo_project"),      <-
//     "gsi1sk": S("$Task#employee_e42069#task_id_1a2b3c4d"),     <-
//     "gsi2pk": S("$TaskService#Task#employee_e42069"),          <-
//     "gsi2sk": S("$Task#project_foo_project#task_id_1a2b3c4d"), <-
//     "employee": S("e42069"),
//     "project": S("foo_project"),
//     "description": S("nothin' but chillin' 20's"),
//     "task_id": S("1a2b3c4d"),
// }
```

The following example shows a practical use-case interacting with DynamoDB
client:

```rust
use aws_sdk_dynamodb::Client;

#[tokio::main]
async fn main() {
    // local configuration
    let client = Client::new(
        &aws_config::from_env()
            .endpoint_url("http://localhost:8000")
            .region("us-east-1")
            .load()
            .await,
    );

    // `create` example
    let task = Task {
        project: "foo_project".to_string(),
        employee: "e42069".to_string(),
        description: "nothin' but chillin' 20's".to_string(),
        some_metadata: "baz".to_string(),
        ..Default::default()
    };

    client
        .put_item()
        .table_name(Task::table_name())
        .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            (
                "#pk".to_string(),
                task.index_key(Index::Primary, Key::Hash).field,
            ),
            (
                "#sk".to_string(),
                task.index_key(Index::Primary, Key::Range).field,
            ),
        ])))
        .set_item(Some(task.into())) // <- pass in struct directly using .into()
        .send()
        .await
        .unwrap();

    // `query` example
    let keys = Task {
        task_id: "a145d3f8-4420-4c22-9178-00240102048a".to_string(),
        project: "foo_project".to_string(),
        employee: "e42069".to_string(),
        ..Default::default()
    }
    .index_keys_av(Index::Primary);

    let task_query = client
        .query()
        .table_name(Task::table_name())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), keys.hash.field),
            ("#sk".to_string(), keys.range.field),
        ])))
        .set_expression_attribute_values(Some(HashMap::from([
            (":pk".to_string(), keys.hash.composite),
            (":sk".to_string(), keys.range.composite),
        ])))
        .send()
        .await
        .unwrap();

    let items = TaskItems::from(task_query.items().unwrap()).items(); // returns `Vec<Task>`
}
```

<!-- Read the full docs on [docs.rs](https://docs.rs/deez/0.1.0/deez/struct.Deez.html) -->
