use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct ImageAggregation {
    pub id: i32,
    pub uri: String,
}
