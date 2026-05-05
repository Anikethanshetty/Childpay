pub mod auth;
pub mod user;
pub mod card;
pub mod wallet;
pub mod transaction;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
