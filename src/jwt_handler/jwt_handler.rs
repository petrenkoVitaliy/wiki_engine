use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use std::env;

use super::error::{error_wrapper::ErrorWrapper, formatted_error::FmtError};
use super::schema::jwt::Claims;

pub struct JwtHandler;

impl JwtHandler {
    fn get_secret() -> String {
        env::var("JWT_SECRET").expect(&FmtError::EmptyValue("JWT_SECRET").fmt())
    }

    fn get_expiration() -> i64 {
        let live_sec = env::var("JWT_LIVE_SEC")
            .expect(&FmtError::EmptyValue("JWT_LIVE_SEC").fmt())
            .parse::<i64>()
            .expect(&FmtError::FailedToProcess("JWT_LIVE_SEC").fmt());

        Utc::now()
            .checked_add_signed(chrono::Duration::seconds(live_sec))
            .expect(&FmtError::FailedToProcess("timestamp").fmt())
            .timestamp()
    }

    pub fn encode_jwt(user_id: i32, role_id: i32) -> Result<String, ErrorWrapper> {
        let secret = Self::get_secret();
        let expiration = Self::get_expiration();

        let claims = Claims {
            user_id,
            role_id,
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

    pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
        let secret = Self::get_secret();
        let token = token.trim_start_matches("Bearer").trim();

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(err.kind().to_owned()),
        }
    }
}
