use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Clone)]
pub struct Wallet {
    pub id : Uuid,
    pub user_id : Option<Uuid>,
    pub card_id : Option<Uuid>,
    pub balance : BigDecimal,
    pub locked_balance : BigDecimal,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}

