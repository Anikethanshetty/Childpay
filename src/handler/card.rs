use std::sync::Arc;

use aws_sdk_s3::{Client, error::SdkError, operation::put_object::PutObjectError, primitives::ByteStream};
use image::{Luma, DynamicImage, ImageFormat};
use qrcode::QrCode;
use std::io::Cursor;
use base64::{Engine as _, alphabet, engine};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{AppState, 
    database::{
        card_pins::CardPinsExt, cards::CardExt, users::UserExt, wallets::WalletsExt}, 
        dtos::{
            Response, card::
            {CardCreateDto, CardsData, CheckCardPinDto, CreateCardData, CreateCardResponse, FilterCardDto, GetCardsResponse}, user::{UserData, UserResponse}, wallet::FilterWalletDto}, error::{ErrorMessage, HttpError}, middleware::JwtAuthMiddleware, utils::pin_hasher::{pin_compare, pin_hash}};

type HmacSha256 = Hmac<Sha256>;

pub async fn create_card(
    Extension(app_state) : Extension<Arc<AppState>>,
    Extension(user) : Extension<JwtAuthMiddleware>,
    Json(body) : Json<CardCreateDto>
)-> Result<impl IntoResponse,HttpError>{

        body.validate().map_err(|e|HttpError::bad_request(e.to_string()))?;

        let card = app_state
                            .db_client
                            .create_card(
                                user.user.id, 
                                body.childname, 
                                body.phonenumber, 
                            ).await.map_err(|_|HttpError::server_error(ErrorMessage::ServerError.to_string()))?;
           
            let card_id = card.id.to_string();

            let signature = sign(&card_id, &app_state.env.qr_secret);

            let payload = format!("{}:{}", card_id, signature);

            let alphabet = alphabet::Alphabet::new(
                "+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                ).map_err(|e| {
                     HttpError::server_error(e.to_string())
                })?;

            
        let crazy_config = engine::GeneralPurposeConfig::new()
                    .with_decode_allow_trailing_bits(true)
                    .with_encode_padding(true)
                    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

        let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);



        let encoded = crazy_engine.encode(payload);
                        
        let code = QrCode::new(encoded.as_bytes())
                            .map_err(|_| HttpError::server_error("QR generation failed".to_string()))?;

        let image = code.render::<Luma<u8>>().build();

        let mut buffer = Vec::new();
                             DynamicImage::ImageLuma8(image)
                            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
                            .map_err(|_| HttpError::server_error("Image conversion failed".to_string()))?;
        
        let key = format!("cards/{}.png", card_id);

         upload(

                &app_state.s3_client,
                &app_state.env.s3_bucket,
                buffer,
                &key,

        ).await
         .map_err(|e| HttpError::server_error(e.to_string()))?;

         let qr_url = format!(
                    "https://{}.s3.{}.amazonaws.com/{}",
                    app_state.env.s3_bucket,
                    app_state.env.aws_region,
                    key
            );
        
        let updated_card = app_state
                        .db_client
                        .update_card(
                            card.id,
                             qr_url
                        ).await
                        .map_err(|e|{
                            HttpError::server_error(e.to_string())
                        })?;
    
        let wallet = app_state
                        .db_client
                        .create_wallet(
                            None, 
                            Some(card.id)
                        ).await.map_err(|e| {
                            HttpError::server_error(e.to_string())
                        })?;


                    let hased_pin = pin_hash(body.card_pin).map_err(|e|{
                        HttpError::server_error(ErrorMessage::HashingError.to_string())
                    })?;

                    app_state
                            .db_client
                            .create_card_pin(card.id, hased_pin)
                            .await.map_err(|e|{
                                let message = format!("Unable to create card pin {}",e.to_string());
                                HttpError::new(message,StatusCode::INTERNAL_SERVER_ERROR)
                            })?;

        Ok((
            StatusCode::OK,
            Json(
                CreateCardResponse {
                    status : "success",
                    data : CreateCardData {
                        card : FilterCardDto::filter_card(&updated_card),
                        wallet : FilterWalletDto::filter_wallet(&wallet)
                    }
                }
        )
    ))

}


async fn upload(
    client: &Client,
    bucket_name: &str,
    file_bytes: Vec<u8>,
    key: &str,
) -> Result<(), SdkError<PutObjectError>> {

    let res = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(ByteStream::from(file_bytes))
        .content_type("image/png")
        .send()
        .await;

    match res {
        Ok(output) => {
            println!("S3 UPLOAD SUCCESS: {:?}", output);
            Ok(())
        }
        Err(e) => {
            println!("S3 UPLOAD ERROR: {:?}", e); 
            Err(e)
        }
    }

   
}


fn sign(data: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");

    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}



    pub async fn get_user_cards(
        Extension(app_state) : Extension<Arc<AppState>>,
        Extension(user) : Extension<JwtAuthMiddleware>
    ) -> Result<impl IntoResponse,HttpError> {

        let cards  = app_state
                .db_client
                .get_cards(user.user.id)
                .await
                .map_err(|e|{
                    HttpError::not_found(e.to_string())
                })?;

        let filtered_cards : Vec<FilterCardDto> = cards.iter()
                            .map(|card|FilterCardDto::filter_card(card))
                            .collect();
        
        Ok(
            (
                StatusCode::FOUND,
                Json(
                    GetCardsResponse {
                        status : "success",
                        data : {
                            CardsData {
                                card : filtered_cards 
                            }
                        }
                    }
                )
            )
        )
    }

    pub async fn check_user_card_pin(
        Extension(app_state) : Extension<Arc<AppState>>,
        Json(body): Json<CheckCardPinDto>
    ) -> Result<impl IntoResponse,HttpError> {

        body.validate().map_err(|e| {
            HttpError::bad_request(e.to_string())
        })?;

        let hased_card_pin  = app_state
                                .db_client
                                .check_card_pin(body.card_id)
                                .await
                                .map_err(|_|{
                                    HttpError::not_found("Cannot find the card")
                                })?;

        let card_pin = pin_compare(
                            body.card_pin, 
                            &hased_card_pin.hashed_pin
                        ).map_err(|e|{
                            HttpError::new(e.to_string(),StatusCode::BAD_REQUEST)
                    })?;

        if card_pin {
            Ok(
                (
                    StatusCode::OK,
                   Json( 
                    Response {
                        status : "success",
                        message : "Correct Pin".to_string()
                    })
                )
            )
        } else {
            Err(HttpError::unauthorized(ErrorMessage::WrongPin.to_string()))
        }

        
}