use axum::{Router, routing::post};

use crate::handler::wallets::{add_card_money,create_vendor_wallet_pin,recive_money};

pub fn wallet_handler() -> Router {
    Router::new()
    .route("/add_money", post(add_card_money))
    .route("/create/vendor", post(create_vendor_wallet_pin))
    .route("/recive/money", post(recive_money))
}