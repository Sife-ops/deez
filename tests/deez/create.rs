use deez::*;
use std::collections::HashMap;
// use std::sync::Arc;

use super::super::schemas::foo::{init, Foo};
use super::super::schemas::make_client;

#[tokio::test]
async fn create() {
    init().await;

    let c = make_client().await;

    let f = Foo {
        foo_string_1: "aaa".to_string(),
        foo_string_2: "bbb".to_string(),
        foo_string_3: "ccc".to_string(),
        foo_string_4: "ddd".to_string(),
        foo_string_5: "eee".to_string(),
        foo_string_6: "fff".to_string(),
        ..Default::default()
    };

    c.put_item()
        .table_name(f.table())
        .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            (
                "#pk".to_string(),
                f.index_key(Index::Primary, Key::Hash).field,
            ),
            (
                "#sk".to_string(),
                f.index_key(Index::Primary, Key::Range).field,
            ),
        ])))
        .set_item(Some(f.clone().into()))
        .send()
        .await
        .unwrap();

    c.query()
        .table_name(f.table())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            (
                "#pk".to_string(),
                f.index_key(Index::Primary, Key::Hash).field,
            ),
            (
                "#sk".to_string(),
                f.index_key(Index::Primary, Key::Range).field,
            ),
        ])))
        // .set_expression_attribute_values(Some(HashMap::from([
        //     (":pk".to_string(), f.index_key(Index::Primary, Key::Hash).composite)
        // ])))
        ;

    // let r = d
    //     .query(
    //         PRIMARY,
    //         &Foo {
    //             foo_string_1: "foo".to_string(),
    //             ..Default::default()
    //         },
    //     )
    //     .unwrap()
    //     .build()
    //     .send()
    //     .await
    //     .unwrap();

    // let rr = Foo::from_av_map_slice(r.items().unwrap()).unwrap();
    // let rrr = rr.first().unwrap();
    // // println!("{:#?}", rrr);

    // assert_eq!(rrr.foo_string_1, "foo");
    // assert_eq!(rrr.foo_string_2, "bar");
    // assert_eq!(rrr.foo_string_3, "baz");
    // assert_eq!(rrr.foo_string_4, "fooz");
    // assert_eq!(rrr.foo_usize, 33);
    // assert_eq!(rrr.foo_bool, false);
}

// #[tokio::test]
// async fn create_with_threads() {
//     init().await;
//     let d = make_deez_arc().await;

//     let mut v = Vec::with_capacity(10);
//     for i in 0..10 {
//         let dd = Arc::clone(&d);
//         v.push(tokio::spawn(async move {
//             dd.create(&Foo {
//                 foo_string_1: i.to_string(),
//                 foo_string_2: "foo".to_string(),
//                 foo_string_3: i.to_string(),
//                 foo_string_4: i.to_string(),
//                 ..Default::default()
//             })
//             .unwrap()
//             .send()
//             .await
//             .unwrap();
//         }));
//     }

//     for i in v {
//         i.await.unwrap();
//     }

//     let r = d
//         .query(
//             GSI1,
//             &Foo {
//                 foo_string_2: "foo".to_string(),
//                 ..Default::default()
//             },
//         )
//         .unwrap()
//         .build()
//         .send()
//         .await
//         .unwrap();

//     let rr = Foo::from_av_map_slice(r.items().unwrap()).unwrap();
//     // println!("{:#?}", rr);

//     assert_eq!(rr.len(), 10);
// }
