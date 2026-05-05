use sqlx::{Error};
use uuid::Uuid;
use async_trait::async_trait;
use crate::{database::DBClient, models::users::{User,Role}};
 


#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id :Uuid,
    ) -> Result<User,Error>;
}

#[async_trait]
impl UserExt for DBClient{
     async fn get_user(
        &self,
        user_id : Uuid,
    ) -> Result<User,Error> {

        let row = sqlx::query_as!(
                User,
                r#"
                    SELECT id, username, password, email, phonenumber, user_role as "user_role:Role", created_at, updated_at FROM users
                    WHERE id = $1
                "#, 
                user_id
            ).fetch_optional(&self.pool).await?;

            let row = row.ok_or_else(|| {
                Error::RowNotFound
            })?;

            Ok(row)       
    }
}
