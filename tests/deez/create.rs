use deez::*;
use std::collections::HashMap;

use super::super::schemas::foo::{init, Foo, FooItems};
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

    let ff = f.index_keys_av(Index::Primary);
    let q = c
        .query()
        .table_name(f.table())
        .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
        .set_expression_attribute_names(Some(HashMap::from([
            ("#pk".to_string(), ff.hash.field),
            ("#sk".to_string(), ff.range.field),
        ])))
        .set_expression_attribute_values(Some(HashMap::from([
            (":pk".to_string(), ff.hash.composite),
            (":sk".to_string(), ff.range.composite),
        ])))
        .send()
        .await
        .unwrap();

    let fff: FooItems = q.items().unwrap().into();
    let ffff = fff.0.first().unwrap();

    println!("{:#?}", ffff);

    assert_eq!(ffff.foo_string_1, "aaa");
    assert_eq!(ffff.foo_string_2, "bbb");
    assert_eq!(ffff.foo_string_3, "ccc");
    assert_eq!(ffff.foo_string_4, "ddd");
    assert_eq!(ffff.foo_string_5, "eee");
    assert_eq!(ffff.foo_string_6, "");
    assert_eq!(ffff.foo_num1, 69.0);
    assert_eq!(ffff.foo_bool1, true);
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
