use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Clone)]
pub struct WithdrawalPin {
    pub id : Uuid,
    pub vendor_id : Uuid,
    pub hashed_pin : String,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}