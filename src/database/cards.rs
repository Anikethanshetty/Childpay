
use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;
use crate::{models::cards::CardStatus};

use crate::{database::DBClient, models::cards::Card};


#[async_trait]
pub trait CardExt {
    async fn create_card(
        &self,
        parent_id : Uuid,
        childname : String,
        phonenumber : String,
    ) -> Result<Card,Error>;

    async fn get_cards(
        &self,
        parent_id : Uuid,
    ) -> Result<Vec<Card>,Error>;

    async fn get_card(
        &self,
        card_id : Uuid,
    ) -> Result<Card,Error>;

    async fn update_card(
         &self,
         card_id : Uuid,
         qr_code_link : String
    ) -> Result<Card,Error>;
}

#[async_trait]
impl CardExt for DBClient {
    async fn create_card(
        &self,
        parent_id : Uuid,
        childname : String,
        phonenumber : String,
    ) -> Result<Card,Error> {

        let card = sqlx::query_as!(
            Card,
            r#"
                INSERT INTO cards(parent_id,cardname,phonenumber)
                VALUES ($1,$2,$3)
                RETURNING id,parent_id,cardname,phonenumber,card_status "card_status:CardStatus",card_qr_code,created_at,updated_at
            "#,
            parent_id,
            childname,
            phonenumber,
        ).fetch_one(&self.pool).await?;
        Ok(card)
    }

  async fn get_cards(
        &self,
        parent_id : Uuid,
    ) -> Result<Vec<Card>,Error> {

        let cards = sqlx::query_as!(
            Card,
            r#"
                SELECT id,parent_id, cardname, phonenumber, card_status as "card_status:CardStatus", card_qr_code, created_at, updated_at
                FROM cards 
                WHERE parent_id = $1
            "#,
            parent_id
        ).fetch_all(&self.pool).await?;

        if cards.len() == 0 {
            Err(Error::RowNotFound)
        }else {
            Ok(cards)
        }
        
    }

    async fn get_card(
        &self,
        card_id : Uuid,
    ) -> Result<Card,Error> {

        let card = sqlx::query_as!(
            Card,
            r#"
                SELECT id,parent_id, cardname, phonenumber, card_status as "card_status:CardStatus", card_qr_code, created_at, updated_at
                FROM cards 
                WHERE id = $1
            "#,
            card_id
        ).fetch_optional(&self.pool).await?;

       let card =  card.ok_or_else(|| {
        Error::RowNotFound
       })?;

       Ok(card)
    }

    async fn update_card(
         &self,
         card_id : Uuid,
         qr_code_link : String
    ) -> Result<Card,Error> {
        
        let card = sqlx::query_as!(
            Card,
            r#"
                UPDATE cards SET 
                card_qr_code = $1
                WHERE id = $2
                RETURNING id,parent_id,cardname,phonenumber,card_status "card_status:CardStatus",card_qr_code,created_at,updated_at
            "#,
            qr_code_link,
            card_id
        ).fetch_optional(&self.pool).await?;

        let upadated_card = card.ok_or_else(||{
            Error::RowNotFound
        })?;

        Ok(upadated_card)
    }
}