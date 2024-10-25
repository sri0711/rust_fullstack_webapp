use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub age: i8,
    pub email: Option<String>,
    pub phone: String,
    pub password: String
}
