use super::ping_db;
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
