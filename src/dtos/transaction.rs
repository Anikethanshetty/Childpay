use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use once_cell::sync::Lazy;
use regex::Regex;

static PIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{6}$").unwrap());


pub fn validate_amount(amount: &str) -> Result<(), ValidationError> {
    match amount.parse::<f64>() {
        Ok(val) if val > 0.0 => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_amount");
            err.message = Some("Amount must be a valid number greater than 0".into());
            Err(err)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PaymentRequestDto {
    
    pub card_id: Uuid,

    #[validate(
        length(min = 1, message = "Amount is required"),
        custom(function = "validate_amount")
    )]
    pub amount: String,

    #[validate(
        length(equal = 6, message = "PIN must be exactly 6 digits"),
        regex(path = *PIN_REGEX, message = "PIN must contain only digits")
    )]
    pub card_pin: String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct WithdrawalRequestDto {
    pub user_id : Uuid,
     #[validate(
        length(min = 1, message = "Amount is required"),
        custom(function = "validate_amount")
    )]
    pub amount: String,
}


