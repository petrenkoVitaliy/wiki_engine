use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticlePatchBody {
    pub enabled: bool,
}

pub struct ArticlePatchDto {
    pub id: i32,
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct ArticleCreateRelationsDto {
    pub content: String,
    pub language: String,
    pub name: String,
}

impl ArticleCreateRelationsDto {
    pub fn from_json(json_dto: Json<ArticleCreateRelationsDto>) -> ArticleCreateRelationsDto {
        ArticleCreateRelationsDto {
            content: json_dto.content.to_string(),
            language: json_dto.language.to_string(),
            name: json_dto.name.to_string(),
        }
    }
}
