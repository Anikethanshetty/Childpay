use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Clone,sqlx::Type,Serialize,Deserialize)]
#[sqlx(type_name = "card_status", rename_all = "lowercase")]
pub enum CardStatus {
    Active,
    Blocked
}

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Clone)]
pub struct Card {
    pub id : Uuid,
    pub parent_id : Uuid,
    pub cardname : String,
    pub phonenumber : String,
    pub card_status : CardStatus,
    pub card_qr_code : Option<String>,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}