use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ResponseString {
    pub status: String,
}
