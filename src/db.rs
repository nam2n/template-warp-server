//! Provides method to create database client

use mongodb::options::ClientOptions;
use mongodb::{error::Error, Client, Database};

///
/// Returns [`Database`](https://docs.rs/mongodb/0.9.2/mongodb/struct.Database.html) client
///
/// `uri` connection string with parameters
///
/// Please note that srv resolution is not supported with tokio because of nested runtime in mongodb driver
///
/// `db_name` name of the database
///
/// `user_pass` as optional (username, password) when applicable
///
/// The method could return a [`Error`](https://docs.rs/mongodb/0.9.2/mongodb/error/struct.Error.html)
pub fn connect<S: AsRef<str> + std::fmt::Display>(
    uri: &str,
    db_name: &str,
    user_pass: &Option<(S, S)>,
) -> Result<Database, Error> {
    let client_options = if let Some(val) = user_pass {
        let uri_parts: Vec<&str> = uri.split("//").collect();
        let uri_with_creds = format!("{}//{}:{}@{}", uri_parts[0], val.0, val.1, uri_parts[1]);

        ClientOptions::parse(&uri_with_creds)
    } else {
        ClientOptions::parse(uri)
    }?;

    Client::with_options(client_options).map(|c| c.database(db_name.as_ref()))
}
