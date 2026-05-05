use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate,ValidationError};

use crate::models::users::Role::{self,Parent,Vendor};

pub fn validate_password_complexity(password: &str) -> Result<(),ValidationError> {
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let number = Regex::new(r"[0-9]").unwrap();
    let special = Regex::new(r"[!@#$%^&*(),.{}?<>:]").unwrap();

     if !uppercase.is_match(password) {
        return Err(ValidationError::new(
            "must contain at least one uppercase letter",
        ));
    }
    if !lowercase.is_match(password) {
        return Err(ValidationError::new(
            "must contain at least one lowercase letter",
        ));
    }
    if !number.is_match(password) {
        return Err(ValidationError::new("must contain at least one number"));
    }
    if !special.is_match(password) {
        return Err(ValidationError::new(
            "must contain at least one special character",
        ));
    };
    Ok(())
}



#[derive(Debug,Serialize,Deserialize,Clone,Validate)]
pub struct RegisterUserDto {
    #[validate(length(min=1,message = "name is required"))]
    pub username: String,

    #[validate(length(min=1,message = "email is required"),
        email(message = "Email is invalid"))]
    pub email: String,

    #[validate(length(min=8,max=64,message = "Password must be between 8 to 64 characters"),
    custom(function = "validate_password_complexity"))]
    pub password: String,

    #[validate(length(min = 10, max = 10, message = "Enter a valid phonenumber"))]
    pub phonenumber : String,

    pub user_role : Role
}

#[derive(Debug,Serialize,Deserialize,Clone,Validate)]
pub struct LoginUserDto {
    #[validate(length(min=1,message = "email is required"),
        email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min=8,max=64,message = "Password must be between 8 to 64 characters"))]
    pub password: String
}



