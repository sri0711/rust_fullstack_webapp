// service level routes
pub mod routes {
    pub mod user_route;
    pub mod authentication_route;
}

// service level controllers
pub mod controllers {
    pub mod user_controller;
    pub mod authentication_controller;
}

// service middlewares
pub mod middlewares {
    pub mod auth_middleware;
    pub mod common_middlewares;
    pub mod route_logger;
}

// app level configurations
pub mod app {
    pub mod config;
    pub mod connection;
    pub mod responder;
}

// mongo db schema
pub mod models {
    pub mod user_model;
}

pub mod helpers {
    pub mod jwt_helper;
    pub mod utils;
}