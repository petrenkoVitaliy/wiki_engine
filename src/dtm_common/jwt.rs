use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, JsonSchema, Deserialize)]
pub struct TokenDto {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtDto {
    pub user_id: i32,
    pub exp: usize,
}
