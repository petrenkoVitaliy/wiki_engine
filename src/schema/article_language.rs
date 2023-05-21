use chrono::NaiveDateTime;
use rocket::serde::Serialize;

use super::article_version::ArticleVersionAggregation;
use super::language::LanguageAggregation;

#[derive(Debug)]
pub struct CreateArticleLanguageDto {
    pub name: String,
    pub article_id: i32,
    pub language_id: i32,
}

#[derive(Serialize)]
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
