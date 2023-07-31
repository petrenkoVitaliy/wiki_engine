use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::trait_common::DtoConvert;

use super::dto::{ArticleCreateRelationsDto, ArticlePatchDto};
use super::repository::entity::article::ArticleType;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ArticlePatchBody {
    pub enabled: bool,
}

impl DtoConvert<ArticlePatchDto> for ArticlePatchBody {
    type TParams = (i32, i32);

    fn into_dto(self, (id, user_id): Self::TParams) -> ArticlePatchDto {
        ArticlePatchDto {
            id,
            user_id,
            archived: None,
            enabled: Some(self.enabled),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticleCreateRelationsBody {
    pub content: String,
    pub language: String,
    pub name: String,
    pub article_type: ArticleType,
}

impl DtoConvert<ArticleCreateRelationsDto> for ArticleCreateRelationsBody {
    type TParams = i32;

    fn into_dto(self, user_id: Self::TParams) -> ArticleCreateRelationsDto {
        ArticleCreateRelationsDto {
            user_id,
            content: self.content,
            language: self.language,
            name: self.name,
            article_type: self.article_type,
        }
    }
}
