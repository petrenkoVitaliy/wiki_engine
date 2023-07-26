use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::ArticleType;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticlePatchBody {
    pub enabled: bool,
}

pub struct ArticlePatchDto {
    pub id: i32,
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub user_id: i32,
}

pub struct ArticleCreateDto {
    pub article_type: ArticleType,
    pub user_id: i32,
}

pub struct ArticleCreateRelationsDto {
    pub content: String,
    pub language: String,
    pub name: String,
    pub article_type: ArticleType,
    pub user_id: i32,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct ArticleCreateRelationsBody {
    pub content: String,
    pub language: String,
    pub name: String,
    pub article_type: ArticleType,
}

impl ArticleCreateRelationsBody {
    pub fn to_dto(
        json: Json<ArticleCreateRelationsBody>,
        user_id: i32,
    ) -> ArticleCreateRelationsDto {
        ArticleCreateRelationsDto {
            user_id,
            content: json.content.to_string(),
            language: json.language.to_string(),
            name: json.name.to_string(),
            article_type: json.article_type,
        }
    }
}
