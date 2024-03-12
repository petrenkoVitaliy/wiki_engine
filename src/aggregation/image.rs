use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct ImageAggregation {
    pub id: i32,
    pub uri: String,
}
