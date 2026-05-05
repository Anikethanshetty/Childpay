use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{dtos::wallet::FilterWalletDto, models::cards::{Card, CardStatus}};

static PIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{6}$").unwrap());


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterCardDto {
    pub id : Uuid,
    pub parent_id : Uuid,
    pub cardname : String,
    pub phonenumber : String,
    pub card_status : CardStatus,
    pub card_qr_code : Option<String>,
} 

impl FilterCardDto {
    pub fn filter_card(card: &Card) -> Self {     
        Self { 
                id : card.id,
                parent_id : card.parent_id,
                cardname : card.cardname.clone(),
                phonenumber : card.phonenumber.clone(),
                card_status : card.card_status.clone(),
                card_qr_code : card.card_qr_code.clone(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CardCreateDto {
    #[validate(length(min = 1,message = "Childname is requiured"))]
    pub childname : String,
    #[validate(length(min = 10, max = 10, message = "Enter a valid phonenumber" ))]
    pub phonenumber : String,

    #[validate(length(equal = 6, message = "PIN must be exactly 6 digits"),
           regex(path = *PIN_REGEX, message = "PIN must contain only digits"))]
    pub card_pin: String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CheckCardPinDto {

    pub card_id : Uuid,
    
   #[validate(length(equal = 6, message = "PIN must be exactly 6 digits"),
           regex(path = *PIN_REGEX, message = "PIN must contain only digits"))]
    pub card_pin: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct GetCardsResponse {
    pub status : &'static str,
    pub data : CardsData
}


#[derive(Debug,Serialize,Deserialize)]
pub struct CardsData {
    pub card : Vec<FilterCardDto>
}

#[derive(Serialize)]
pub struct CreateCardResponse {
    pub status: &'static str,
    pub data: CreateCardData,
}


#[derive(Debug,Serialize)]
pub struct CreateCardData {
    pub card: FilterCardDto,
    pub wallet: FilterWalletDto, 
}
