use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

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
