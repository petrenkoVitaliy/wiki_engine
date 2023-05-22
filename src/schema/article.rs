use chrono::NaiveDateTime;
use rocket::serde::{json::Json, Deserialize, Serialize};

use super::article_language::ArticleLanguageAggregation;

#[derive(Deserialize)]
#[serde()]
pub struct ArticleCreateDto {
    pub content: String,
    pub language: String,
    pub name: String,
}

impl ArticleCreateDto {
    pub fn from_json(json_dto: Json<ArticleCreateDto>) -> ArticleCreateDto {
        ArticleCreateDto {
            content: json_dto.content.to_string(),
            language: json_dto.language.to_string(),
            name: json_dto.name.to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde()]
pub struct ArticlePatchBody {
    pub enabled: bool,
}

pub struct ArticlePatchDto {
    pub id: i32,
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
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
