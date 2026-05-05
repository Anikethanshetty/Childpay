use axum::{Router, routing::{post,get}};
use crate::handler::card::{create_card,check_user_card_pin,get_user_cards};


pub fn card_handler() ->Router {
    Router::new()
    .route("/create", post(create_card))
    .route("/get", get(get_user_cards))
    .route("/check/pin", get(check_user_card_pin))
}