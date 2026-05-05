use std::sync::Arc;

use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;

use crate::AppState;
use crate::middleware::auth_middleware;

pub mod auth;
pub mod card;
pub mod wallet;


pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth::auth_handler())
        .nest("/card", card::card_handler().layer(middleware::from_fn(auth_middleware)))
        .nest("/wallet", wallet::wallet_handler().layer(middleware::from_fn(auth_middleware)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_routes)

} 