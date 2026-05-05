use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize,sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed
}

#[derive(Debug,Clone,Serialize,Deserialize,sqlx::Type)]
#[sqlx(type_name = "types", rename_all = "lowercase")]
pub enum TransactionType {
    Recharge,
    Payment,
    Withdrawal
}


#[derive(Debug,Serialize,Deserialize,Clone,sqlx::FromRow)]
pub struct Transaction {
    pub id : Uuid,
    pub from_wallet_id : Uuid,
    pub to_wallet_id : Uuid,
    pub amount : BigDecimal,
    pub transaction_type : TransactionType,
    pub transaction_status : TransactionStatus,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}