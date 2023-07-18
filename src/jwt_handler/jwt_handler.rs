use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;

use super::error::{error_wrapper::ErrorWrapper, formatted_error::FmtError};
use super::jwt::Claims;

pub struct JwtHandler;

impl JwtHandler {
    pub fn encode_jwt(user_id: i32) -> Result<String, ErrorWrapper> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); // TODO rename

        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(5)) // TODO env
            .expect("invalid timestamp") // TODO normal error
            .timestamp();

        let claims = Claims {
            user_id,
            exp: expiration as usize,
        };

        let header = Header::new(Algorithm::HS512);

        match encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(jwt_string) => Ok(jwt_string),
            Err(_) => FmtError::FailedToProcess("jwt").error(),
        }
    }

    pub fn decode_jwt(token: String) -> Result<Claims, ErrorWrapper> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); // TODO rename
        let token = token.trim_start_matches("Bearer").trim();

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token) => Ok(token.claims),
            Err(_) => FmtError::FailedToProcess("jwt").error(),
        }
    }
}
