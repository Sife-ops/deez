// use chrono::prelude::*;

use anyhow::{Ok, Result};
// use aws_sdk_dynamodb::types::AttributeValue;
use deez::*;
use std::collections::HashMap;

use super::super::schemas::foo::{init, Task};
use super::super::schemas::make_client;

#[tokio::test]
async fn query_macro() -> Result<()> {
    init().await;
    let client = make_client().await;

    for i in 1..11 {
        let task = Task {
            task_id: format!("task_id_{}", i),
            project: "project_name".to_string(),
            employee: "employee_name".to_string(),
            description: format!("description_{}", i),
            ..Default::default()
        };

        client
            .put_item()
            .table_name(Task::table_name())
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), task.primary_key(Key::Hash).field),
                ("#sk".to_string(), task.primary_key(Key::Range).field),
            ])))
            .set_item(Some(task.into()))
            .send()
            .await?;
    }

    ////////////////////////////////////////////////////////////////////////////

    // // filter - begins with
    // {
    //     let task_keys = Task {
    //         project: "project_name".to_string(),
    //         ..Default::default()
    //     }
    //     .gsi1_keys();
    //     // .index_keys_av(Index::Gsi1);
    //     let items = exec!(
    //         with_index!(
    //             with_filter!(
    //                 query!(
    //                     client;
    //                     Task::table_name();
    //                     "#pk = :pk";
    //                     "#pk" => task_keys.hash.field,
    //                     "#description" => "description".to_string();
    //                     ":pk" => task_keys.hash.composite,
    //                     ":description" => AttributeValue::S("description_1".to_string())
    //                 ),
    //                 "begins_with(#description, :description)"
    //             ),
    //             Task::gsi1_name()
    //         ) => TaskItems
    //     );
    //     println!("{:#?}", items);
    //     assert_eq!(items.len(), 2);
    // }

    Ok(())
}

// #[tokio::test]
// async fn query() {
//     init().await;
//     let d = make_deez().await;

//     d.batch_write()
//         .put(vec![
//             &Foo {
//                 foo_string_1: "foo".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 2, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             },
//             &Foo {
//                 foo_string_1: "bar".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             },
//             &Foo {
//                 foo_string_1: "baz".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             },
//             &Foo {
//                 foo_string_1: "for".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 5, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             },
//             &Foo {
//                 foo_string_1: "far".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 6, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             },
//         ])
//         .unwrap()
//         .build()
//         .unwrap()
//         .send()
//         .await
//         .unwrap();

//     // pk only
//     {
//         let a = d
//             .query(
//                 GSI1,
//                 &Foo {
//                     foo_string_2: "deez".to_string(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .build()
//             .send()
//             .await
//             .unwrap();

//         let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
//         // println!("{:#?}", b);

//         assert_eq!(b.len(), 5);
//     }

//     // sk gte
//     {
//         let a = d
//             .query(
//                 GSI1,
//                 &Foo {
//                     foo_string_2: "deez".to_string(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .gte(&Foo {
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             })
//             .unwrap()
//             .build()
//             .send()
//             .await
//             .unwrap();

//         let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
//         // println!("{:#?}", b);

//         assert_eq!(b.len(), 3);
//     }

//     // sk lt
//     {
//         let a = d
//             .query(
//                 GSI1,
//                 &Foo {
//                     foo_string_2: "deez".to_string(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .lt(&Foo {
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: Utc
//                     .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
//                     .unwrap()
//                     .to_rfc3339(),
//                 ..Default::default()
//             })
//             .unwrap()
//             .build()
//             .send()
//             .await
//             .unwrap();

//         let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
//         // println!("{:#?}", b);

//         assert_eq!(b.len(), 2);
//     }

//     // sk between
//     {
//         let a = d
//             .query(
//                 GSI1,
//                 &Foo {
//                     foo_string_2: "deez".to_string(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .between(
//                 &Foo {
//                     foo_string_3: "composed_index".to_string(),
//                     foo_string_4: Utc
//                         .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
//                         .unwrap()
//                         .to_rfc3339(),
//                     ..Default::default()
//                 },
//                 &Foo {
//                     foo_string_3: "composed_index".to_string(),
//                     foo_string_4: Utc
//                         .with_ymd_and_hms(2015, 1, 5, 3, 4, 5)
//                         .unwrap()
//                         .to_rfc3339(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .build()
//             .send()
//             .await
//             .unwrap();

//         let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
//         // println!("{:#?}", b);

//         assert_eq!(b.len(), 3);
//     }

//     init().await;

//     d.batch_write()
//         .put(vec![
//             &Foo {
//                 foo_string_1: "foo".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: "foobar".to_string(),
//                 ..Default::default()
//             },
//             &Foo {
//                 foo_string_1: "bar".to_string(),
//                 foo_string_2: "deez".to_string(),
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: "foobaz".to_string(),
//                 ..Default::default()
//             },
//         ])
//         .unwrap()
//         .build()
//         .unwrap()
//         .send()
//         .await
//         .unwrap();

//     // sk begins
//     {
//         let a = d
//             .query(
//                 GSI1,
//                 &Foo {
//                     foo_string_2: "deez".to_string(),
//                     ..Default::default()
//                 },
//             )
//             .unwrap()
//             .begins(&Foo {
//                 foo_string_3: "composed_index".to_string(),
//                 foo_string_4: "foo".to_string(),
//                 ..Default::default()
//             })
//             .unwrap()
//             .build()
//             .send()
//             .await
//             .unwrap();

//         let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
//         // println!("{:#?}", b);

//         assert_eq!(b.len(), 2);
//     }
// }
