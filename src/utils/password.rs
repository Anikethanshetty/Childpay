
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString, Error as PasswordHashError,
    },
    Argon2
};

use crate::error::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;

pub fn hash(password: impl AsRef<[u8]>) -> Result<String, ErrorMessage> {
    let password = password.as_ref();

    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::PasswordTooShort(MIN_PASSWORD_LENGTH));
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password, &salt)
        .map_err(|e| match e {
            PasswordHashError::Password => ErrorMessage::HashingError,
            _ => ErrorMessage::HashingError,
        })?;

    Ok(hash.to_string())
}

pub fn  compare(password: impl AsRef<[u8]>, hashed_password: &str) -> Result<bool, ErrorMessage> {
    let password = password.as_ref();
    
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::PasswordTooShort(MIN_PASSWORD_LENGTH));
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|_| ErrorMessage::InvalidHashFormat)?;

    match Argon2::default().verify_password(password, &parsed_hash) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(_) => Err(ErrorMessage::HashingError),
    }
}