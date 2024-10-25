use std::env;

pub struct Environment {
    pub mongodb_uri: String,
    pub mongodb_database_name: String,
}

impl  Environment {
    pub fn from_env() ->Self {
        let mongo_database_url = env::var("MONGO_DATABASE_URL").unwrap_or_else(|_|"mongodb://localhost:27017".to_owned());
        let mongo_database_name = env::var("MONGO_DATABASE_NAME").unwrap_or_else(|_|"sample".to_owned());

        Environment {
            mongodb_uri: mongo_database_url,
            mongodb_database_name: mongo_database_name
        }
    }
}