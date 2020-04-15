use super::common;
use bson::doc;
use log::error;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn ping(
    db: Arc<mongodb::Database>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("healthz").and(common::with_db(db)).and_then(
        |db: Arc<mongodb::Database>| async move {
            let r: Result<_, Rejection> = match db.run_command(doc! { "ping": 1 }, None) {
                Ok(res) => {
                    if res.contains_key("ok") {
                        Ok(warp::reply::with_status(
                            warp::reply::json(&()),
                            warp::http::StatusCode::OK,
                        ))
                    } else {
                        error!("Database ping responded with error: {:?}", res);
                        Ok(common::error_response(
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            "server_error",
                            "Health check failed",
                        ))
                    }
                }
                Err(e) => {
                    error!("Error while pinging the database: {}", e);
                    Ok(common::error_response(
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "server_error",
                        "Health check failed",
                    ))
                }
            };

            return r;
        },
    )
}
