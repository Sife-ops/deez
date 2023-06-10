use deez::DeezEntity;

use super::super::schemas::foo::{init, Foo, GSI1};
use super::super::schemas::make_deez;

#[tokio::test]
async fn batch_write() {
    init().await;
    let d = make_deez().await;

    // put
    {
        d.batch_write()
            .put(vec![
                &Foo {
                    foo_string_1: "foo".to_string(),
                    foo_string_2: "foo".to_string(),
                    foo_string_3: "foo".to_string(),
                    ..Default::default()
                },
                &Foo {
                    foo_string_1: "bar".to_string(),
                    foo_string_2: "foo".to_string(),
                    foo_string_3: "bar".to_string(),
                    ..Default::default()
                },
                &Foo {
                    foo_string_1: "baz".to_string(),
                    foo_string_2: "foo".to_string(),
                    foo_string_3: "baz".to_string(),
                    ..Default::default()
                },
            ])
            .unwrap()
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

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

        assert_eq!(rr.len(), 3);
    }

    // delete/put
    {
        d.batch_write()
            .put(vec![&Foo {
                foo_string_1: "fooz".to_string(),
                foo_string_2: "foo".to_string(),
                foo_string_3: "fooz".to_string(),
                ..Default::default()
            }])
            .unwrap()
            .delete(vec![
                &Foo {
                    foo_string_1: "foo".to_string(),
                    ..Default::default()
                },
                &Foo {
                    foo_string_1: "bar".to_string(),
                    ..Default::default()
                },
                &Foo {
                    foo_string_1: "baz".to_string(),
                    ..Default::default()
                },
            ])
            .unwrap()
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

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

        assert_eq!(rr.len(), 1);
    }
}
