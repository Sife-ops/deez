use deez::{DeezEntity, DeezMeta};
use std::sync::Arc;

use super::super::schemas::foo::{init, Foo, GSI1, PRIMARY};
use super::super::schemas::{make_deez, make_deez_arc};

#[tokio::test]
async fn create() {
    init().await;
    let d = make_deez().await;

    d.create(&Foo {
        foo_string_1: "foo".to_string(),
        foo_string_2: "bar".to_string(),
        foo_string_3: "baz".to_string(),
        foo_string_4: "fooz".to_string(),
        ..Foo::generated()
    })
    .unwrap()
    .send()
    .await
    .unwrap();

    let r = d
        .query(
            PRIMARY,
            &Foo {
                foo_string_1: "foo".to_string(),
                ..Default::default()
            },
        )
        .unwrap()
        .build()
        .send()
        .await
        .unwrap();

    let rr = Foo::from_av_map_slice(r.items().unwrap()).unwrap();
    let rrr = rr.first().unwrap();
    // println!("{:#?}", rrr);

    assert_eq!(rrr.foo_string_1, "foo");
    assert_eq!(rrr.foo_string_2, "bar");
    assert_eq!(rrr.foo_string_3, "baz");
    assert_eq!(rrr.foo_string_4, "fooz");
    assert_eq!(rrr.foo_usize, 33);
    assert_eq!(rrr.foo_bool, false);
}

#[tokio::test]
async fn create_with_threads() {
    init().await;
    let d = make_deez_arc().await;

    let mut v = Vec::with_capacity(10);
    for i in 0..10 {
        let dd = Arc::clone(&d);
        v.push(tokio::spawn(async move {
            dd.create(&Foo {
                foo_string_1: i.to_string(),
                foo_string_2: "foo".to_string(),
                foo_string_3: i.to_string(),
                foo_string_4: i.to_string(),
                ..Default::default()
            })
            .unwrap()
            .send()
            .await
            .unwrap();
        }));
    }

    for i in v {
        i.await.unwrap();
    }

    let r = d
        .query(
            GSI1,
            &Foo {
                foo_string_2: "foo".to_string(),
                ..Default::default()
            },
        )
        .unwrap()
        .build()
        .send()
        .await
        .unwrap();

    let rr = Foo::from_av_map_slice(r.items().unwrap()).unwrap();
    // println!("{:#?}", rr);

    assert_eq!(rr.len(), 10);
}
