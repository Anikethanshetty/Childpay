use axum::{Router, routing::post};

use crate::handler::auth::{register,login};


pub fn auth_handler() -> Router {
    Router::new()
    .route("/register", post(register))
    .route("/login", post(login))
}