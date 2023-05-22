use chrono::NaiveDateTime;
use rocket::serde::Serialize;

// TODO rename
#[derive(Debug)]
pub struct ArticleVersionCreateDto {
    pub version: i32,
    pub content: String,

    pub article_language_id: i32,
}

#[derive(Serialize)]
pub struct ArticleVersionAggregation {
    pub id: i32,
    pub version: i32,
    pub content: String,
    pub enabled: bool,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub article_language_id: i32,
}
