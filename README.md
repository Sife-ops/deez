A DynamoDB abstraction for Rust
==========================================================
[![Crates.io](https://img.shields.io/crates/v/deez.svg)](https://crates.io/crates/deez)

Deez is a DynamoDB abstraction for implementing Single Table Design easily,
inspired by [ElectroDB](https://github.com/tywalch/electrodb).

## Getting Started

Define a schema for your entities:

```rust
use aws_sdk_dynamodb::types::AttributeValue;
use deez::{DeezEntity, DeezError, DeezMeta, Index, IndexKeys, Key, Meta};
use std::collections::HashMap;
use uuid::Uuid;

const TABLE_NAME: &str = "TaskTable";
const PRIMARY: Index = Index::Primary;
const GSI1: Index = Index::Gsi1("gsi1");
const GSI2: Index = Index::Gsi2("gsi2");

#[derive(Debug, Default, DeezEntity)]
struct Task {
    task_id: String,
    project: String,
    employee: String,
    description: String,
}

impl DeezMeta for Task {
    fn meta(&self) -> Meta {
        Meta {
            table: TABLE_NAME,
            service: "TaskService",
            entity: "Task",
        }
    }

    fn indexes(&self) -> HashMap<Index, IndexKeys> {
        let mut m = HashMap::new();
        m.insert(
            PRIMARY,
            IndexKeys {
                partition_key: Key {
                    field: "pk",
                    composite: vec!["task_id"],
                },
                sort_key: Key {
                    field: "sk",
                    composite: vec!["project", "employee"],
                },
            },
        );
        m.insert(
            GSI1,
            IndexKeys {
                partition_key: Key {
                    field: "gsi1pk",
                    composite: vec!["project"],
                },
                sort_key: Key {
                    field: "gsi1sk",
                    composite: vec!["employee", "task"],
                },
            },
        );
        m.insert(
            GSI2,
            IndexKeys {
                partition_key: Key {
                    field: "gsi2pk",
                    composite: vec!["employee"],
                },
                sort_key: Key {
                    field: "gsi2sk",
                    composite: vec!["project", "task"],
                },
            },
        );
        m
    }

    fn generated() -> Self {
        Task {
            task_id: Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }
}
```

Create a client to interact with your entities:

```rust
use aws_sdk_dynamodb::Client;
use deez::Deez;

#[tokio::main]
async fn main() {
    // local configuration
    let ddb_client = Client::new(
        &aws_config::from_env()
            .endpoint_url("http://localhost:8000")
            .region("us-east-1")
            .load()
            .await,
    );

    let deez = Deez::new(ddb_client);

    // create
    deez.create(&Task {
        project: "foo_project".to_string(),
        employee: "Bill Bar".to_string(),
        description: "nothin' but chillin' 20's".to_string(),
        ..Task::generated()
    })
    .unwrap()
    .send()
    .await
    .unwrap();

    // convenience macro for simple queries
    let result = query_Task!(
        deez,
        GSI1,
        Task {
            project: "foo_project".to_string(),
            ..Default::default()
        }
    );

    println!("{:#?}", result);
}
```

Read the full list of methods on [docs.rs](https://docs.rs/deez/0.1.0/deez/struct.Deez.html)
