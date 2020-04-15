use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

mod common;
mod healthz;

pub fn all(db: mongodb::Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db = Arc::new(db);
    warp::get()
        .and(healthz::ping(db.clone()))
        .with(warp::log("template-warp-server"))
}
