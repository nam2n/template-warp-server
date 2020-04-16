use bson::Document;
use mongodb::error::Error;
use mongodb::options::SelectionCriteria;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub(crate) trait DbInterface {
    fn run_command(
        &self,
        command: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Document, Error>;
}

pub struct DbInterfaceImpl<'a>(pub &'a mongodb::Database);

impl<'a> DbInterface for DbInterfaceImpl<'a> {
    fn run_command(
        &self,
        command: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Document, Error> {
        self.0.run_command(command, selection_criteria)
    }
}

pub fn with_db(
    db: Arc<mongodb::Database>,
) -> impl Filter<Extract = (Arc<mongodb::Database>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn error_response(
    status: warp::http::StatusCode,
    error_code: &str,
    error_msg: &str,
) -> warp::reply::WithStatus<warp::reply::Json> {
    warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "error": error_code,
            "message": error_msg
        })),
        status,
    )
}
