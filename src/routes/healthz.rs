use super::common::{self, DbInterface};
use crate::routes::common::DbInterfaceImpl;
use bson::doc;
use log::error;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn ping(
    db: Arc<mongodb::Database>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("healthz").and(common::with_db(db)).and_then(
        |db: Arc<mongodb::Database>| async move {
            let r: Result<_, Rejection> = ping_db(DbInterfaceImpl(&db))
                .map(|_| {
                    warp::reply::with_status(warp::reply::json(&()), warp::http::StatusCode::OK)
                })
                .or_else(|s| {
                    error!("Error in health check: {}", s);
                    Ok(common::error_response(
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "server_error",
                        "Health check failed",
                    ))
                });

            return r;
        },
    )
}

fn ping_db<DB: DbInterface>(db: DB) -> Result<(), String> {
    match db.run_command(doc! { "ping": 1 }, None) {
        Ok(res) => {
            if res.contains_key("ok") {
                Ok(())
            } else {
                Err(format!("Database ping responded with error: {:?}", res))
            }
        }
        Err(e) => Err(format!("Error while pinging the database: {}", e)),
    }
}

#[cfg(test)]
#[path = "./healthz_test.rs"]
mod tests;
