use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::article_version::ArticleVersionAggregation;
use super::language::LanguageAggregation;

#[derive(Deserialize, JsonSchema)]
pub struct ArticleLanguageCreateBody {
    pub content: String,
    pub name: String,
}

#[derive(Debug)]
pub struct ArticleLanguageCreateRelationsDto {
    pub content: String,
    pub name: String,
    pub language_code: String,
    pub article_id: i32,
}

#[derive(Debug)]
pub struct ArticleLanguageCreateDto {
    pub name: String,
    pub article_id: i32,
    pub language_id: i32,
}

#[derive(Serialize, JsonSchema)]
pub struct ArticleLanguageAggregation {
    pub id: i32,
    pub name: String,

    pub enabled: bool,
    pub archived: bool,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub language: LanguageAggregation,
    pub versions: Vec<ArticleVersionAggregation>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ArticleLanguagePatchBody {
    pub enabled: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct ArticleLanguagePatchDto {
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub name: Option<String>,
}
