pub mod auth;
pub mod users;
pub mod cards;
pub mod card_pins;
pub mod wallets;
pub mod transactions;
pub mod withdrawal_pins;

use sqlx::{Pool, Postgres, Transaction,Error};


#[derive(Debug,Clone)]
pub struct DBClient {
    pool: Pool<Postgres>
}

impl DBClient {
    pub fn new(pool:Pool<Postgres>) -> Self {
        Self {pool}
    }
}

impl DBClient {
    pub async fn begin(&self) -> Result<Transaction<'_, Postgres>, Error> {
        self.pool.begin().await
    }
}