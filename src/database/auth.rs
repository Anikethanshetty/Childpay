use crate::{database::DBClient, models::users::{Role, User}};
use sqlx::Error;
use async_trait::async_trait;

#[async_trait]
pub trait AuthExt {

    async fn register(
        &self,
        username : String,
        password : String,
        email : String,
        phonenumber : String,
        user_role : Role
    ) -> Result<User,Error> ;
 
     async fn login(
        &self,
        email:String,
    ) -> Result<Option<User>,Error>;
}

#[async_trait]
impl AuthExt for DBClient  {

    async fn register(
            &self,
            username : String,
            password : String,
            email : String,
            phonenumber : String,
            user_role : Role
        ) -> Result<User,Error> {

            let username = username.into();
            let email = email.into();
            let password = password.into();
            let phonenumber = phonenumber.into();
        
        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username,email,password,phonenumber,user_role)
                VALUES ($1,$2,$3,$4,$5::roles)
                RETURNING id,username,password,email,phonenumber,user_role as "user_role:Role",created_at,updated_at 
            "#,
            username,
            email,
            password,
            phonenumber,
            user_role as Role
        ).fetch_one(&self.pool)
        .await?;

        Ok(user)

    }


    async fn login(
            &self,
            email:String,
        ) -> Result<Option<User>,Error> {
        
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT 
                    id,
                    username,
                    password,
                    email,
                    phonenumber,
                    user_role as "user_role:Role",
                    created_at,
                    updated_at  
                FROM users
                WHERE email = $1
            "#,
            email
        ).fetch_optional(&self.pool).await?;

        Ok(user)
    }
}