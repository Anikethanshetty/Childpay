use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::models::wallets::Wallet;

static PIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{6}$").unwrap());


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterWalletDto {
    pub id : Uuid,
    pub user_id : Option<Uuid>,
    pub card_id : Option<Uuid>,
    pub balance : BigDecimal,
    pub locked_balance : BigDecimal,
} 

impl FilterWalletDto {
    pub fn filter_wallet(wallet: &Wallet) -> Self {     
        Self { 
                id : wallet.id,
                user_id : wallet.user_id.clone(),
                card_id : wallet.card_id.clone(),
                balance : wallet.balance.clone(),
                locked_balance : wallet.locked_balance.clone(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct GetCardWalletDto {
    //card_id or user_id
    pub id : Uuid,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct WalletResponseDto {
    pub status : &'static str,
    pub data : WalletData
}

#[derive(Debug,Serialize,Deserialize)]
pub struct WalletData {
    pub wallet : FilterWalletDto
}


#[derive(Debug,Serialize)]
pub struct WalletPinResponseDto {
    pub status : &'static str,
    pub is_pin_available : bool 
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  ReciveMoneyToWallet {

    #[validate(length(min = 1, message = "QR code link is required"))]
    pub qr_code_link: String,

    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: u64,

    #[validate(length(min = 1, message = "Pin is required"))]
    pub card_pin : String
}


#[derive(Debug,Serialize)]
pub struct ReciveMoneyResponseDto {
    pub status : &'static str,
    pub data : ReciveMoneyData
}


#[derive(Debug,Serialize)]
pub struct ReciveMoneyData {
    pub user_wallet : FilterWalletDto,
    pub vendor_wallet : FilterWalletDto
}


#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CreateVendorPin {
    #[validate(length(equal = 6, message = "PIN must be exactly 6 digits"),
    regex(path = *PIN_REGEX, message = "PIN must contain only digits"))]
    pub pin : String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct WithdrawMoneyRequestDto {
    #[validate(range(min = 1,message = "Amount must be greater than 0"))]
    pub amount : u64,
    #[validate(length(equal = 6, message = "PIN must be exactly 6 digits"),
        regex(path = *PIN_REGEX, message = "PIN must contain only digits"))]
    pub pin : String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct AddMoneyRequestDto {
    #[validate(range(min = 1,message = "Amount must be greater than 0"))]
    pub amount : u64,
    
    pub card_id : Uuid
}