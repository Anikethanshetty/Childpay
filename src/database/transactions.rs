#[allow(unused)]

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::{Error};
use uuid::Uuid;

use crate::{database::{DBClient}, models::transactions::{Transaction, TransactionStatus, TransactionType}};

#[async_trait]
pub trait TransactionExt {
    async fn create_transaction(
        &self,
        from_wallet_id : Uuid,
        to_wallet_id : Uuid,
        amount : BigDecimal,
        transaction_status : TransactionStatus,
        transaction_type : TransactionType
    ) ->  Result<Transaction,Error>;

    async fn get_transactions(
        &self,
        wallet_id : Uuid,
        transaction_type : Option<TransactionType>,
        transaction_status : Option<TransactionStatus>,
        page : Option<u64>,
        limit : Option<u64>
    ) -> Result<Vec<Transaction>,Error>;

    async fn update_transaction(
        &self,
        wallet_id : Uuid,
        tarnsaction_status : TransactionStatus

    ) -> Result<Option<Transaction>,Error>;
}


#[async_trait]
impl TransactionExt for DBClient {
       async fn create_transaction(
        &self,
        from_wallet_id : Uuid,
        to_wallet_id : Uuid,
        amount : BigDecimal,
        transaction_status : TransactionStatus,
        transaction_type : TransactionType
    ) ->  Result<Transaction,Error> {

        let transaction = sqlx::query_as!(
            Transaction,
            r#"
                INSERT INTO transactions ( from_wallet_id, to_wallet_id, amount, transaction_status, transaction_type)
                VALUES ( $1, $2, $3, $4::transaction_status, $5::types )
                RETURNING id, from_wallet_id, to_wallet_id, amount, transaction_status as "transaction_status:TransactionStatus", transaction_type as "transaction_type:TransactionType", created_at, updated_at
            "#,
            from_wallet_id,
            to_wallet_id,
            amount,
            transaction_status as TransactionStatus,
            transaction_type  as TransactionType
        ).fetch_one(&self.pool).await?;

        Ok(transaction)
    }

    async fn get_transactions(
        &self,
        wallet_id : Uuid,
        transaction_type : Option<TransactionType>,
        transaction_status : Option<TransactionStatus>,
        page : Option<u64>,
        limit : Option<u64>
    ) -> Result<Vec<Transaction>,Error> {

        // let mut qb = QueryBuilder::new("SELECT id, from_wallet_id, to_wallet_id, amount, transaction_status,transaction_type, created_at, updated_at FROM transactions WHERE ");

       
        todo!()
    }

    async fn update_transaction(
        &self,
        wallet_id : Uuid,
        tarnsaction_status : TransactionStatus
    ) -> Result<Option<Transaction>,Error> {
        todo!()
    }
}