use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};

use colored::*;

pub async fn simple_log(req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    let path: String = req.uri().to_string();
    let response = next.run(req).await;
    let status: u16 = response.status().as_u16();
    println!(
        "api path is: {} status code {}",
        path,
        status.to_string().red()
    );
    Ok(response)
}
