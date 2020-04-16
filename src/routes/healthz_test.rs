use super::{ping, ping_db};
use bson::doc;
use mongodb::options::SelectionCriteria;
use std::sync::Arc;

#[test]
fn success_on_ok_db_response() {
    let mut mock = crate::routes::common::MockDbInterface::new();

    mock.expect_run_command()
        .times(1)
        .withf(
            |command: &bson::Document, selection_criteria: &Option<SelectionCriteria>| {
                *command == doc! { "ping": 1 } && selection_criteria.is_none()
            },
        )
        .returning(|_, _| Ok(doc! { "ok": 1 }));

    let actual = ping_db(mock);

    assert_eq!(actual, Ok(()))
}

#[test]
fn failure_on_db_error() {
    let mut mock = crate::routes::common::MockDbInterface::new();

    mock.expect_run_command()
        .times(1)
        .withf(
            |command: &bson::Document, selection_criteria: &Option<SelectionCriteria>| {
                *command == doc! { "ping": 1 } && selection_criteria.is_none()
            },
        )
        .returning(|_, _| {
            let err_kind = Arc::new(mongodb::error::ErrorKind::ResponseError {
                message: "".to_string(),
            });
            Err(mongodb::error::Error { kind: err_kind })
        });

    let actual = ping_db(mock);

    assert!(actual.is_err());
}

#[test]
fn failure_on_unknown_db_response() {
    let mut mock = crate::routes::common::MockDbInterface::new();

    mock.expect_run_command()
        .times(1)
        .withf(
            |command: &bson::Document, selection_criteria: &Option<SelectionCriteria>| {
                *command == doc! { "ping": 1 } && selection_criteria.is_none()
            },
        )
        .returning(|_, _| Ok(doc! {"ko": 1, "23": 0}));

    let actual = ping_db(mock);

    assert!(actual.is_err());
}

#[tokio::test]
async fn test_healthz_path() {
    let db = crate::db::connect::<&str>("mongodb://localhost", "mydatabase", &None).unwrap();

    let filter = ping(Arc::new(db));

    assert!(
        warp::test::request()
            .path("/healthz")
            .matches(&filter)
            .await
    );

    assert!(!warp::test::request().path("/health").matches(&filter).await);

    assert!(!warp::test::request().path("/").matches(&filter).await);

    assert!(
        !warp::test::request()
            .path("/healthz/abc")
            .matches(&filter)
            .await
    );

    assert!(
        !warp::test::request()
            .path("/healthz/123")
            .matches(&filter)
            .await
    );

    assert!(
        !warp::test::request()
            .path("/healthz/ab1")
            .matches(&filter)
            .await
    );
}
