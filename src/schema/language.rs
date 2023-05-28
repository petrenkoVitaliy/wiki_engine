use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Clone, Serialize, JsonSchema)]
pub struct LanguageAggregation {
    pub id: i32,
    pub code: String,
}
