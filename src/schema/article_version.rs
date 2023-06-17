use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::version_content::VersionContentAggregation;

#[derive(Serialize, JsonSchema)]
pub struct ArticleVersionAggregation {
    pub id: i32,
    pub version: i32,
    pub enabled: bool,

    pub content: VersionContentAggregation,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub article_language_id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct ArticleVersionCreateBody {
    pub content: String,
}

#[derive(Debug)]
pub struct ArticleVersionCreateDto {
    pub version: i32,
    pub content_id: i32,

    pub article_language_id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct ArticleVersionPatchBody {
    pub enabled: bool,
}

pub struct ArticleVersionPatchDto {
    pub enabled: bool,
}

pub struct ArticleVersionsSearchDto {
    pub article_languages_ids: Option<Vec<i32>>,
    pub ids: Option<Vec<i32>>,
}

pub struct ArticleVersionsJoinSearchDto {
    pub article_languages_ids: Option<Vec<i32>>,
    pub version_gt: Option<i32>,
}

pub struct ArticleVersionSearchDto {
    pub id: Option<i32>,
    pub article_languages_ids: Option<Vec<i32>>,
}
