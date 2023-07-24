use chrono::Utc;

use super::aggregation::article::ArticleAggregation;
use super::ArticleType;

use super::schema::article::ArticleCreateRelationsDto;

use super::article_language::{ArticleLanguageMockHandler, ArticleLanguageMockOptions};

pub struct ArticleMockOptions {
    pub content: String,
    pub language: String,
    pub name: String,
    pub enabled: bool,
    pub archived: bool,
    pub article_type: ArticleType,
}

impl ArticleMockOptions {
    pub fn from_creation_dto(creation_dto: ArticleCreateRelationsDto) -> Self {
        Self {
            enabled: true,
            archived: false,

            content: String::from(creation_dto.content),
            language: String::from(creation_dto.language),
            name: String::from(creation_dto.name),
            article_type: creation_dto.article_type,
        }
    }
}

pub struct ArticleMockHandler {}

impl ArticleMockHandler {
    pub fn get_article_aggregation(create_dto: ArticleMockOptions) -> ArticleAggregation {
        ArticleAggregation {
            id: 0,
            enabled: create_dto.enabled,
            archived: create_dto.archived,
            article_type: create_dto.article_type,
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            languages: vec![
                ArticleLanguageMockHandler::get_article_language_aggregation(
                    &ArticleLanguageMockOptions {
                        content: String::from(create_dto.content),
                        name: String::from(create_dto.name),
                        language: String::from(create_dto.language),
                        enabled: true,
                        archived: false,
                    },
                ),
            ],
        }
    }
}
