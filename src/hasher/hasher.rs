use argon2::{self, Config};
use std::env;

use super::error::{ErrorWrapper, FmtError};

const ARGON_SECRET_ENV: &str = "ARGON_SECRET";

pub struct Hasher;

impl Hasher {
    pub fn verify_encoded(password: String, stored_password: String) -> Result<bool, ErrorWrapper> {
        match argon2::verify_encoded(&stored_password, password.as_bytes()) {
            Ok(is_correct) => Ok(is_correct),
            Err(_) => FmtError::FailedToProcess("password").error(),
        }
    }

    pub fn hash_password(password: String) -> Result<String, ErrorWrapper> {
        let salt = env::var(ARGON_SECRET_ENV).expect(&FmtError::EmptyValue(ARGON_SECRET_ENV).fmt());

        let config = Config::default();
        match argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config) {
            Ok(hash) => Ok(hash),
            Err(_) => FmtError::FailedToProcess("password").error(),
        }
    }
}
