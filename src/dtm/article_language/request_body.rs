use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::trait_common::DtoConvert;

use super::dto::{ArticleLanguageCreateRelationsDto, ArticleLanguagePatchDto};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticleLanguageCreateRelationsBody {
    pub content: String,
    pub name: String,
}

impl DtoConvert<ArticleLanguageCreateRelationsDto> for ArticleLanguageCreateRelationsBody {
    type TParams = (i32, i32, String);

    fn into_dto(
        self,
        (user_id, article_id, language_code): Self::TParams,
    ) -> ArticleLanguageCreateRelationsDto {
        ArticleLanguageCreateRelationsDto {
            user_id,
            article_id,
            language_code,
            content: self.content,
            name: self.name,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticleLanguagePatchBody {
    pub enabled: Option<bool>,
    pub name: Option<String>,
}

impl DtoConvert<ArticleLanguagePatchDto> for ArticleLanguagePatchBody {
    type TParams = i32;

    fn into_dto(self, user_id: Self::TParams) -> ArticleLanguagePatchDto {
        ArticleLanguagePatchDto {
            user_id,
            archived: None,
            enabled: self.enabled,
            name: self.name,
        }
    }
}
