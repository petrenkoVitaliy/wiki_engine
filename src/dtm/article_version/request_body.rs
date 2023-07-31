use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::trait_common::DtoConvert;

use super::dto::{ArticleVersionCreateRelationsDto, ArticleVersionPatchDto};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ArticleVersionCreateRelationsBody {
    pub content: String,
}

impl DtoConvert<ArticleVersionCreateRelationsDto> for ArticleVersionCreateRelationsBody {
    type TParams = i32;

    fn into_dto(self, user_id: Self::TParams) -> ArticleVersionCreateRelationsDto {
        ArticleVersionCreateRelationsDto {
            user_id,
            content: self.content,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ArticleVersionPatchBody {
    pub enabled: bool,
}

impl DtoConvert<ArticleVersionPatchDto> for ArticleVersionPatchBody {
    type TParams = i32;

    fn into_dto(self, user_id: Self::TParams) -> ArticleVersionPatchDto {
        ArticleVersionPatchDto {
            user_id,
            enabled: self.enabled,
        }
    }
}
