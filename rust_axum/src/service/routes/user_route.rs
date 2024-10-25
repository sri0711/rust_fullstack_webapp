use crate::service::controllers::user_controller;
use axum::{routing::*, Router};

pub async fn user_routes() -> Router {
    let list_users = Router::new().route("/list", get(|| user_controller::user_list()));
    let view_users = Router::new().route("/view", get(|| async { "view user" }));
    let create_users = Router::new().route("/create", post(|| user_controller::create_user()));

    let merged_user_routes = list_users.merge(view_users).merge(create_users);

    Router::new().nest("/users", merged_user_routes)
}
