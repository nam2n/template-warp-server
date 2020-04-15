use log::{info, warn};

mod db;
mod routes;

#[tokio::main]
async fn main() {
    init_env();

    init_logging();

    let db = create_db_client();

    start_server(db).await;
}

fn init_env() {
    match std::env::var("APP_ENV") {
        Ok(_) => {}
        Err(_) => {
            warn!("using development config from .env");
            dotenv::dotenv().expect("No .env found");
        }
    };
}

fn init_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // initializing logger
    env_logger::init();
}

fn create_db_client() -> mongodb::Database {
    let uri = std::env::var("DATABASE_URL").expect("Database URI not defined at DATABASE_URL");

    // add username and password to the database url when available
    let db_userpass = match std::env::var("DATABASE_USER") {
        Ok(db_user) => {
            let db_password = std::env::var("DATABASE_PASSWORD")
                .expect("Database password not defined at DATABASE_PASSWORD");
            Some((db_user, db_password))
        }
        _ => None,
    };

    let db_name =
        std::env::var("DATABASE_NAME").expect("Database name not defined at DATABASE_NAME");

    db::connect(&uri, &db_name, &db_userpass).expect("Database connection failed")
}

async fn start_server(db: mongodb::Database) {
    let port = std::env::var("WEBSERVER_PORT")
        .expect("WEBSERVER_PORT not defined")
        .parse::<u16>()
        .expect("WEBSERVER_PORT is not a number");

    info!("Starting server at port {}", port);
    warp::serve(routes::all(db)).run(([0, 0, 0, 0], port)).await;
}
