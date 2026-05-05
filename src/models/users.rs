use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize,sqlx::Type,PartialEq, Eq)]
#[sqlx(type_name = "roles",rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Parent,
    Vendor,
    Both
} 


#[derive(Debug,Serialize,Deserialize,sqlx::FromRow,Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phonenumber : String,
    pub user_role : Role,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}



