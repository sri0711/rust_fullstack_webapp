// use axum::extract::Json;
use futures_util::TryStreamExt;
use serde_json::Value;
use crate::service;
use service::app::{connection::Database, responder::ApiResponse};
use service::models::user_model::User;


pub async fn user_list() -> ApiResponse<Value> {
    let database = Database::init();
    let users = match database.await.users.find(Default::default()).await {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to fetch users: {:?}", e);
            return ApiResponse {
                success: false,
                message: "Failed to fetch users".to_string(),
                data: None,
                status: Some(500),
            };
        }
    };
    let user_vec: Vec<User> = match users.try_collect().await {
        Ok(u) => u,
        Err(e) => {
            println!("Error while collecting users: {:?}", e);
            return ApiResponse {
                success: false,
                message: "Failed to collect users".to_string(),
                data: None,
                status: Some(500),
            };
        }
    };
    let result = match serde_json::to_value(&user_vec) {
        Ok(u) => u,
        Err(e) => {
            println!("Error while parse users: {:?}", e);
            return ApiResponse {
                success: false,
                message: "Failed to parse users".to_string(),
                data: None,
                status: Some(500),
            };
        }
    };
    ApiResponse {
        success: true,
        message: "user_list".to_string(),
        data: Some(result),
        status: Some(201),
    }
}

pub async fn create_user() -> ApiResponse<String> {
    let database = Database::init();

    let new_user: User = User{
        user_id: "xx1".to_string(),
        name: "sathish".to_string(),
        age: 27,
        phone: "7904739162".to_string(),
        email: Some("m.sathish@gmail.com".to_string()),
        password: "sathish".to_string()
    };
    let res = database.await.users.insert_one(new_user).await;
    println!("{:?}", res);
    ApiResponse {
        success: true,
        message: "user created Successfully".to_string(),
        data: None,
        status: Some(201),
    }
}
