
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString, Error as PasswordHashError,
    },
    Argon2
};

use crate::error::ErrorMessage;

const MAX_PIN_LENGTH: usize = 6;
const MIN_PIN_LENGTH: usize = 6;

pub fn pin_hash(pin: impl AsRef<[u8]>) -> Result<String, ErrorMessage> {
    let pin  = pin.as_ref();

    if pin.is_empty() {
        return Err(ErrorMessage::EmptyPin);
    }
    if pin.len() < MIN_PIN_LENGTH {
        return Err(ErrorMessage::PasswordTooShort(MIN_PIN_LENGTH));
    }
    if pin.len() > MAX_PIN_LENGTH {
        return Err(ErrorMessage::ExceededMaxPinLength(MAX_PIN_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(pin, &salt)
        .map_err(|e| match e {
            PasswordHashError::Password => ErrorMessage::HashingError,
            _ => ErrorMessage::HashingError,
        })?;

    Ok(hash.to_string())
}

pub fn  pin_compare(pin: impl AsRef<[u8]>, hashed_pin: &str) -> Result<bool, ErrorMessage> {
    let pin = pin.as_ref();
    
    if pin.is_empty() {
        return Err(ErrorMessage::EmptyPin);
    }
    if pin.len() < MIN_PIN_LENGTH {
        return Err(ErrorMessage::PinTooShort(MIN_PIN_LENGTH));
    }
    if pin.len() > MAX_PIN_LENGTH {
        return Err(ErrorMessage::ExceededMaxPinLength(MAX_PIN_LENGTH));
    }

    let parsed_hash = PasswordHash::new(hashed_pin)
        .map_err(|_| ErrorMessage::InvalidHashFormat)?;

    match Argon2::default().verify_password(pin, &parsed_hash) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(_) => Err(ErrorMessage::HashingError),
    }
}