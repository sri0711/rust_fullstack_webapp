use axum::{routing::*, Router};
pub async fn authentication_routes() -> Router {
    let login = Router::new().route("/login", post(|| async { "login" }));

    let merged_authentication_routes = login;
    Router::new().nest("/auth", merged_authentication_routes)
}