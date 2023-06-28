use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct ArticleLanguageCreateBody {
    pub content: String,
    pub name: String,
}

pub struct ArticleLanguageCreateDto {
    pub name: String,
    pub article_id: i32,
    pub language_id: i32,
}

pub struct ArticleLanguageCreateRelationsDto {
    pub content: String,
    pub name: String,
    pub language_code: String,
    pub article_id: i32,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct ArticleLanguagePatchBody {
    pub enabled: Option<bool>,
    pub name: Option<String>,
}

pub struct ArticleLanguagePatchDto {
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub name: Option<String>,
}
