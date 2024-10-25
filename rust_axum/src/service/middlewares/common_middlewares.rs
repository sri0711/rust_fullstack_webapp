use axum::{body::Body, extract::Request};
use crate::service::app::responder::ApiResponse;

// 404 error handler
pub async fn not_found_handler(req: Request<Body>) -> ApiResponse<String> {
    let method = req.method();
    let path = req.uri();
    println!("{} path or {} method is not found", &path, &method);
    ApiResponse {
        success: false,
        message: format!("Requesting URI {} or {} Method is not Found", path, method).to_owned(),
        data: None,
        status: None,
    }
}
