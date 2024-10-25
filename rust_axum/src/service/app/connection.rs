use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use crate::service::{app::config, models};
use models::user_model;
use std::process;


pub struct Database {
    pub users: Collection<user_model::User>
}

impl Database {
    pub async fn init () -> Self {
        let env = config::Environment::from_env();
        let mut mongodb_connection_string = ClientOptions::parse(env.mongodb_uri).await.unwrap();
        mongodb_connection_string.app_name = Some("Rust_Backend".to_string());
        let client = Client::with_options(mongodb_connection_string).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
        let sample_database_connection = client.database("sample");
        let users_collection: Collection<user_model::User> = sample_database_connection.collection("users");
        Database {
            users: users_collection
        }
    }
}