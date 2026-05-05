use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{database::DBClient, models::wallets::Wallet};
use sqlx::{Error, QueryBuilder};

#[async_trait]
pub trait WalletsExt {
    async fn create_wallet(
        &self,
        user_id : Option<Uuid>,
        card_id : Option<Uuid>,
    ) -> Result<Wallet,Error>;

     async fn get_wallet(
        &self,
        user_id : Option<Uuid>,
        card_id : Option<Uuid>,
    ) -> Result<Wallet,Error>;

    async fn update_wallet(
         &self,
        user_id : Option<Uuid>,
        card_id : Option<Uuid>,
        balance : Option<BigDecimal>,
        locked_balance : Option<BigDecimal>
    ) -> Result<Wallet,Error>;
   
}

#[async_trait]
impl WalletsExt for DBClient {
     async fn create_wallet(
        &self,
        user_id : Option<Uuid>,
        card_id : Option<Uuid>,
    ) -> Result<Wallet,Error>{

        if (user_id.is_none() && card_id.is_none()) || (user_id.is_some() && card_id.is_some()) {
            return Err(Error::Protocol("Provide either user_id OR card_id (not both)".into()));
        }

        let mut qb = QueryBuilder::new("INSERT INTO wallets ");

        qb.push("(");

        
    if user_id.is_some() {
        qb.push("user_id) VALUES (");
        qb.push_bind(user_id.unwrap());
    } else {
        qb.push("card_id) VALUES (");
        qb.push_bind(card_id.unwrap());
    }

        qb.push(")");
        
        qb.push(" RETURNING id, user_id, card_id, balance, locked_balance, created_at, updated_at");

        let wallet = qb.build_query_as::<Wallet>().fetch_one(&self.pool).await?;

        Ok(wallet)
    }


     async fn get_wallet(
        &self,
        user_id : Option<Uuid>,
        card_id : Option<Uuid>,
    ) -> Result<Wallet,Error> {

          if (user_id.is_none() && card_id.is_none()) || (user_id.is_some() && card_id.is_some()) {
            return Err(Error::Protocol("Provide either user_id OR card_id (not both)".into()));
        }

        let mut qb = QueryBuilder::new("SELECT id, user_id, card_id, balance, locked_balance, created_at, updated_at FROM wallets WHERE ");

        if let Some(user_id) = user_id {
            qb.push("user_id = ").push_bind(user_id);
            
        } else if let Some(card_id) = card_id  {
            qb.push("card_id = ").push_bind(card_id);
        }

        let wallet = qb.build_query_as::<Wallet>().fetch_optional(&self.pool).await?;

        let wallet = wallet.ok_or_else(||{
            Error::RowNotFound
        })?;
       
        Ok(wallet)
    }

     async fn update_wallet(
    &self,
    user_id: Option<Uuid>,
    card_id: Option<Uuid>,
    balance: Option<BigDecimal>,
    locked_balance: Option<BigDecimal>,
) -> Result<Wallet, Error> {

    if (user_id.is_none() && card_id.is_none()) || (user_id.is_some() && card_id.is_some()) {
        return Err(Error::Protocol("Provide either user_id OR card_id".into()));
    }

    if balance.is_none() && locked_balance.is_none() {
        return Err(Error::Protocol("Provide at least one field".into()));
    }

    let mut qb = QueryBuilder::new("UPDATE wallets SET");
    

    if let Some(balance) = balance {
        qb.push(" balance = ");
        qb.push_bind(balance);
    }

    if let Some(locked_balance) = locked_balance {
        qb.push(" locked_balance = ").push_bind(locked_balance);
    }

    

    if let Some(user_id) = user_id {
        qb.push(" WHERE user_id = ").push_bind(user_id);
    } else if let Some(card_id) = card_id {
        qb.push(" WHERE card_id = ").push_bind(card_id);
    }

    qb.push(" RETURNING id, user_id, card_id, balance, locked_balance, created_at, updated_at");

    let wallet = qb
        .build_query_as::<Wallet>()
        .fetch_optional(&self.pool)
        .await?;

    let wallet = wallet.ok_or(Error::RowNotFound)?;

    Ok(wallet)
}
}