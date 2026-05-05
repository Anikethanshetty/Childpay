use std::sync::Arc;

use axum::{Extension, extract::Request, http::header, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, database::users::UserExt, error::{ErrorMessage, HttpError}, models::users::User, utils::token};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtAuthMiddleware {
    pub user: User,
}


pub async fn auth_middleware(
    Extension(app_state) : Extension<Arc<AppState>>,
    mut req : Request,
    next : Next,
) ->Result<impl IntoResponse, HttpError> {
    let headers = req.headers()
                                        .get(header::AUTHORIZATION)
                                        .and_then(|value|value.to_str().ok())
                                        .and_then(|value|{
                                            if value.starts_with("Bearer ") {
                                                Some(value.trim_start_matches("Bearer ").to_string())
                                            }else {
                                                None
                                            }
                                        });
                                   


        let token = headers.ok_or(
                HttpError::unauthorized(
                    ErrorMessage::TokenNotProvided.to_string()
                ))?;

        let token_user_id = match token::decode_token( &token, app_state.env.jwt_secret.as_bytes())   {
            Ok(val ) => {
                val
            }
            Err(_) => {
                return Err(HttpError::unauthorized(
                    ErrorMessage::InvalidToken.to_string()
                ));
            }
        };    

        let user_id = Uuid::parse_str(&token_user_id)
                                        .map_err(|_| {
                                          HttpError::unauthorized(ErrorMessage::InvalidToken.to_string())  
                                        })?;

        let user = app_state
                        .db_client
                        .get_user(user_id).await
                        .map_err(|_| {
                            HttpError::unauthorized(ErrorMessage::InvalidToken.to_string())
                        })?;

        

        req.extensions_mut().insert(JwtAuthMiddleware {
            user : user.clone()
        });

        Ok(next.run(req).await)

}