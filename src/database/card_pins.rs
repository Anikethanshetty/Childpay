use async_trait::async_trait;
use uuid::Uuid;

use crate::{database::DBClient, models::card_pins::CardPin};
use sqlx::Error;

#[async_trait]
pub trait CardPinsExt {
    async fn create_card_pin(
        &self,
        card_id : Uuid,
        card_pin : String
    ) -> Result<CardPin,Error>;

    async fn check_card_pin(
        &self,
        card_id : Uuid
    ) -> Result<CardPin,Error>;
}

#[async_trait]
impl CardPinsExt for DBClient {

    async fn create_card_pin(
        &self,
        card_id:Uuid,
        card_pin:String
    ) -> Result<CardPin,Error> {

        let card = sqlx::query_as!(
            CardPin,
            r#"
                INSERT INTO card_pins (card_id,hashed_pin)
                VALUES ($1,$2)
                RETURNING id,card_id,hashed_pin,created_at,updated_at
            "#,
            card_id,
            card_pin
        ).fetch_one(&self.pool).await?;

        Ok(card)
    }

    async fn check_card_pin(
        &self,
        card_id:Uuid
    ) -> Result<CardPin ,Error> {

        let card = sqlx::query_as!(
            CardPin,
            r#"
                SELECT id, card_id, hashed_pin, created_at,updated_at
                FROM card_pins
                WHERE card_id = $1
            "#,
            card_id
        ).fetch_optional(&self.pool).await?;

        let card_pin = card.ok_or_else(|| {
            Error::RowNotFound
        })?;

        Ok(card_pin)
    }
}