use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

pub use super::ContentType;

#[derive(Serialize, JsonSchema)]
pub struct VersionContentAggregation {
    pub id: i32,
    pub content: String,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

pub struct VersionContentDto {
    pub content: Vec<u8>,
    pub content_type: ContentType,
}
