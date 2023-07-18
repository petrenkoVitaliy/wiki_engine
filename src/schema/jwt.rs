use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: i32,
    pub role_id: i32,
    pub exp: usize,
}
