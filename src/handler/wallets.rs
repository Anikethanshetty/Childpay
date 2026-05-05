use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use bigdecimal::BigDecimal;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use uuid::Uuid;
use validator::Validate;
use base64::{engine, alphabet, Engine as _};

use crate::{AppState, database::{card_pins::CardPinsExt, cards::CardExt, wallets::WalletsExt, withdrawal_pins::WithdrawalPinsExt}, dtos::wallet::{AddMoneyRequestDto, CreateVendorPin, FilterWalletDto, ReciveMoneyData, ReciveMoneyResponseDto, ReciveMoneyToWallet, WalletData, WalletPinResponseDto, WalletResponseDto, WithdrawMoneyRequestDto}, error::{ErrorMessage, HttpError}, middleware::JwtAuthMiddleware, utils::pin_hasher::pin_compare};


type HmacSha256 = Hmac<Sha256>;



pub async fn add_card_money(
    Extension(app_state) : Extension<Arc<AppState>>,
    Json(body) : Json<AddMoneyRequestDto>
) -> Result<impl IntoResponse, HttpError> {

    body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

    let card = app_state
                        .db_client
                        .get_card(body.card_id)
                        .await
                        .map_err(|e|{
                            HttpError::server_error(e.to_string())
                        });

    if let Ok(card) = card {

        let amount = BigDecimal::from(body.amount);

        let updated_wallet = app_state
                                .db_client
                                .update_wallet(
                                    None, 
                                    Some(card.id), 
                                    Some(amount), 
                                    None
                                ).await
                                .map_err(|e| {
                                    HttpError::server_error(e.to_string())
                                })?;

                Ok((
                    StatusCode::OK,
                    Json(
                        WalletResponseDto {
                            status : "success",
                            data : WalletData {
                                wallet : FilterWalletDto::filter_wallet(&updated_wallet)
                            }
                        }
                    )
                ))
    
    } else {
        Err(HttpError::not_found("Card not found"))
    }

    
}

pub async fn recive_money(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body) : Json<ReciveMoneyToWallet>
) -> Result<impl IntoResponse,HttpError> {

    body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

    let alphabet =
    alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    .unwrap();


    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

    let decoded = crazy_engine
                        .decode(body.qr_code_link)
                        .map_err(|_| {
                            HttpError::bad_request("Invalid QR link")
                        })?;

    let payload = String::from_utf8(decoded).map_err(|_| {
        HttpError::server_error(ErrorMessage::ServerError.to_string())
    })?;

    let parts : Vec<&str> = payload.split(":").collect();

    if parts.len() != 2 {
       return  Err(HttpError::bad_request("Invaild Qr!".to_string()));
    }

     let card_id = parts[0];
    let signature = parts[1];

    
    let expected = sign(card_id, &app_state.env.qr_secret);
    
    let card_id  = Uuid::parse_str(card_id).map_err(|e| {
        HttpError::server_error(e.to_string())
    })?;
    
    if expected != signature {
        return Err(HttpError::bad_request("Qr is tampered!"));
    }

    
    
    let card_user = app_state
                .db_client
                .get_card(card_id)
                .await
                .map_err(|e|{
                    HttpError::not_found(e.to_string())
                });

    if let Ok(card) = card_user {
        let wallet = app_state
                .db_client
                .get_wallet(None, Some(card.id))
                .await

                .map_err(|e| {
                    HttpError::not_found(e.to_string())
                })?;

        if wallet.balance < body.amount {
            return Err(HttpError::new("Insufficient Balance", StatusCode::BAD_REQUEST));
        }

        let new_balance  = wallet.balance - body.amount;


        let card_pin  = app_state
                                .db_client
                                .check_card_pin(card.id)
                                .await
                                .map_err(|e| {
                                    HttpError::server_error(e.to_string())
                                })?;

        let pin = pin_compare(body.card_pin, &card_pin.hashed_pin);

        match pin  {
            Ok(pin) => {
                if pin {
                    let updated_balance = app_state
                                .db_client
                                .update_wallet(
                                    None, 
                                    Some(card_id), 
                                    Some(new_balance), 
                                    None
                                ).await.map_err(|e|{
                                    HttpError::server_error(e.to_string())
                                });

        if updated_balance.is_ok() {
            
            let vendor_wallet  = app_state
                                        .db_client
                                        .get_wallet(Some(user.user.id), None)
                                        .await
                                        .map_err(|e|{
                                            HttpError::not_found(e.to_string())
                                        })?;

            let new_vendor_balance = vendor_wallet.balance + body.amount;

            let updated_vendor_wallet = app_state
                                        .db_client
                                        .update_wallet(
                                             Some(user.user.id),
                                             None, 
                                             Some(new_vendor_balance), 
                                             None
                                        ).await.map_err(|e| {
                                            HttpError::not_found(e.to_string())
                                        });

            if updated_vendor_wallet.is_ok() {
                let vendor_wallet = updated_vendor_wallet.map_err(|e| {
                    HttpError::server_error(e.to_string())
                })?;

                let user_wallet = updated_balance.map_err(|e| {
                    HttpError::server_error(e.to_string())
                })?;

               return  Ok((
                            StatusCode::OK,
                            Json(
                                ReciveMoneyResponseDto {
                                    status : "success",
                                    data : {
                                        ReciveMoneyData {
                                            user_wallet : FilterWalletDto::filter_wallet(&user_wallet),
                                            vendor_wallet : FilterWalletDto::filter_wallet(&vendor_wallet)
                                        }
                                    }
                                }
                            )
                        )) ;
            }
            else {
                return Err(HttpError::server_error(ErrorMessage::ServerError.to_string())) ;
            }
        }
        else {
            return Err(HttpError::server_error(ErrorMessage::ServerError.to_string()));
        }
                }
                else {
                    Err(HttpError::not_found("Entered Pin is wrong"))
                }
            }
            Err(e) => {
                Err(HttpError::bad_request(e.to_string()))
            }
        }
                        

    
    } else {
        Err(HttpError::not_found("Card not found"))
    } 

}

fn sign(data: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");

    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}


pub async fn create_vendor_wallet_pin(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body) : Json<CreateVendorPin>
) -> Result<impl IntoResponse, HttpError> {

    body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

                    app_state
                    .db_client
                    .create_withdrawal_pin(user.user.id, body.pin)
                    .await
                    .map_err(|e|{
                        HttpError::server_error(e.to_string())
                    })?;

    Ok((
        StatusCode::OK,
        Json(
            WalletPinResponseDto {
                status : "success",
                is_pin_available : true
            }
        )
     )
    )
}


pub async fn withdraw_money( 
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body) : Json<WithdrawMoneyRequestDto>,
) -> Result<impl IntoResponse,HttpError>{

    body.validate().map_err(|e|{
        HttpError::bad_request(e.to_string())
    })?;

    let vendor_wallet = app_state
                        .db_client
                        .get_wallet(Some(user.user.id), None)
                        .await
                        .map_err(|e| {
                            HttpError::server_error(e.to_string())
                        })?;
    let amount = BigDecimal::from(body.amount);

    if amount > vendor_wallet.balance {
        return Err(HttpError::bad_request("Insufficient Balance"));
    }
    else {
        let balance = vendor_wallet.balance - amount;

        let updated_wallet = app_state
                            .db_client
                            .update_wallet(
                                Some(user.user.id), 
                                None, 
                                Some(balance), 
                                None
                            ).await
                            .map_err(|e| {
                                HttpError::server_error(e.to_string())
                            })?;
            Ok(
                (
                    StatusCode::OK,
                    Json(
                        WalletResponseDto {
                            status : "success",
                            data : WalletData {
                                wallet : FilterWalletDto::filter_wallet(&updated_wallet)
                            }
                        }
                    )
                )
            )
    }

}