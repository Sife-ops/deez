use chrono::prelude::*;
use deez::DeezEntity;

use super::super::schemas::foo::{init, Foo, GSI1};
use super::super::schemas::make_deez;

#[tokio::test]
async fn query() {
    init().await;
    let d = make_deez().await;

    d.batch_write()
        .put(vec![
            &Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 2, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            },
            &Foo {
                foo_string_1: "bar".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            },
            &Foo {
                foo_string_1: "baz".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            },
            &Foo {
                foo_string_1: "for".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 5, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            },
            &Foo {
                foo_string_1: "far".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 6, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            },
        ])
        .unwrap()
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();

    // pk only
    {
        let a = d
            .query(
                GSI1,
                &Foo {
                    foo_string_2: "deez".to_string(),
                    ..Default::default()
                },
            )
            .unwrap()
            .build()
            .send()
            .await
            .unwrap();

        let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
        // println!("{:#?}", b);

        assert_eq!(b.len(), 5);
    }

    // sk gte
    {
        let a = d
            .query(
                GSI1,
                &Foo {
                    foo_string_2: "deez".to_string(),
                    ..Default::default()
                },
            )
            .unwrap()
            .gte(&Foo {
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            })
            .unwrap()
            .build()
            .send()
            .await
            .unwrap();

        let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
        // println!("{:#?}", b);

        assert_eq!(b.len(), 3);
    }

    // sk lt
    {
        let a = d
            .query(
                GSI1,
                &Foo {
                    foo_string_2: "deez".to_string(),
                    ..Default::default()
                },
            )
            .unwrap()
            .lt(&Foo {
                foo_string_3: "sugon".to_string(),
                foo_string_4: Utc
                    .with_ymd_and_hms(2015, 1, 4, 3, 4, 5)
                    .unwrap()
                    .to_rfc3339(),
                ..Default::default()
            })
            .unwrap()
            .build()
            .send()
            .await
            .unwrap();

        let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
        // println!("{:#?}", b);

        assert_eq!(b.len(), 2);
    }

    // sk between
    {
        let a = d
            .query(
                GSI1,
                &Foo {
                    foo_string_2: "deez".to_string(),
                    ..Default::default()
                },
            )
            .unwrap()
            .between(
                &Foo {
                    foo_string_3: "sugon".to_string(),
                    foo_string_4: Utc
                        .with_ymd_and_hms(2015, 1, 3, 3, 4, 5)
                        .unwrap()
                        .to_rfc3339(),
                    ..Default::default()
                },
                &Foo {
                    foo_string_3: "sugon".to_string(),
                    foo_string_4: Utc
                        .with_ymd_and_hms(2015, 1, 5, 3, 4, 5)
                        .unwrap()
                        .to_rfc3339(),
                    ..Default::default()
                },
            )
            .unwrap()
            .build()
            .send()
            .await
            .unwrap();

        let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
        // println!("{:#?}", b);

        assert_eq!(b.len(), 3);
    }

    init().await;

    d.batch_write()
        .put(vec![
            &Foo {
                foo_string_1: "foo".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: "foobar".to_string(),
                ..Default::default()
            },
            &Foo {
                foo_string_1: "bar".to_string(),
                foo_string_2: "deez".to_string(),
                foo_string_3: "sugon".to_string(),
                foo_string_4: "foobaz".to_string(),
                ..Default::default()
            },
        ])
        .unwrap()
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();

    // sk begins
    {
        let a = d
            .query(
                GSI1,
                &Foo {
                    foo_string_2: "deez".to_string(),
                    ..Default::default()
                },
            )
            .unwrap()
            .begins(&Foo {
                foo_string_3: "sugon".to_string(),
                foo_string_4: "foo".to_string(),
                ..Default::default()
            })
            .unwrap()
            .build()
            .send()
            .await
            .unwrap();

        let b = Foo::from_av_map_slice(a.items().unwrap()).unwrap();
        // println!("{:#?}", b);

        assert_eq!(b.len(), 2);
    }
}
