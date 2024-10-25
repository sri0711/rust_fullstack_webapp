// register global module distribution
pub mod service;

// imports
use axum::{middleware, Router};
use service::{middlewares, routes::*};
use middlewares::common_middlewares::not_found_handler;
use dotenv::dotenv;

pub async fn run() {
    dotenv().ok();
    service::app::connection::Database::init().await;
    let merged_routes = user_route::user_routes().await.merge(authentication_route::authentication_routes().await);
    let app = Router::new()
        .nest("/api", merged_routes)
        .route_layer(middleware::from_fn(
            service::middlewares::route_logger::simple_log,
        ))
        .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
