use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use super::article_language::ArticleLanguageAggregation;

#[derive(Deserialize)]
#[serde()]
pub struct CreateArticleDto {
    pub content: String,
    pub language: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct ArticleAggregation {
    pub id: i32,
    pub enabled: bool,
    pub archived: bool,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub languages: Vec<ArticleLanguageAggregation>,
}
