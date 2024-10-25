use axum::extract::Json;
use serde_json::{json, Value};
use crate::service::app::responder::ApiResponse;


struct Login {
    username: String,
    password: String
}
pub async fn login() -> ApiResponse<Value>{

    ApiResponse{
        success: true,
        message: "Login successful".to_owned(),
        data: Some(json!({})),
        status: None
    }
}