use async_trait::async_trait;
use uuid::Uuid;

use crate::{database::DBClient, models::{withdrawal_pin::WithdrawalPin}};
use sqlx::Error;

#[async_trait]
pub trait WithdrawalPinsExt {
    async fn create_withdrawal_pin(
        &self,
        vendor_id : Uuid,
        withdrawal_pin : String
    ) -> Result<WithdrawalPin,Error>;

    async fn check_withdrawal_pin(
        &self,
        vendor_id:Uuid 
    ) -> Result<WithdrawalPin,Error>;
}

#[async_trait]
impl WithdrawalPinsExt for DBClient {

    async fn create_withdrawal_pin(
        &self,
        vendor_id : Uuid,
        withdrawal_pin : String
    ) -> Result<WithdrawalPin,Error> {

        let withdrawal_pin = sqlx::query_as!(
            WithdrawalPin,
            r#"
                INSERT INTO withdrawal_pins (vendor_id,hashed_pin)
                VALUES ($1,$2)
                RETURNING id,vendor_id,hashed_pin,created_at,updated_at
            "#,
            vendor_id,
            withdrawal_pin
        ).fetch_one(&self.pool).await?;

        Ok(withdrawal_pin)
    }

    async fn check_withdrawal_pin(
        &self,
        vendor_id:Uuid
    ) -> Result<WithdrawalPin ,Error> {

        let withdrawal_pin = sqlx::query_as!(
            WithdrawalPin,
            r#"
                SELECT id, vendor_id, hashed_pin, created_at,updated_at
                FROM withdrawal_pins
                WHERE vendor_id = $1
            "#,
            vendor_id
        ).fetch_optional(&self.pool).await?;

        let withdrawal_pin = withdrawal_pin.ok_or_else(|| {
            Error::RowNotFound
        })?;

        Ok(withdrawal_pin)
    }
}