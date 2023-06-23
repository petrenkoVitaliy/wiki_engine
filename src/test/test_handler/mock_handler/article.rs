use chrono::Utc;

use super::aggregation::{
    article::ArticleAggregation, article_language::ArticleLanguageAggregation,
    article_version::ArticleVersionAggregation, language::LanguageAggregation,
    version_content::VersionContentAggregation,
};

use super::schema::article::ArticleCreateRelationsDto;

pub struct ArticleMockOptions {
    pub content: String,
    pub language: String,
    pub name: String,
    pub enabled: bool,
    pub archived: bool,
}

impl ArticleMockOptions {
    pub fn from_creation_dto(creation_dto: ArticleCreateRelationsDto) -> Self {
        Self {
            enabled: true,
            archived: false,

            content: String::from(creation_dto.content),
            language: String::from(creation_dto.language),
            name: String::from(creation_dto.name),
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
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            languages: vec![ArticleLanguageAggregation {
                id: 0,
                name: String::from(create_dto.name),
                enabled: true,
                archived: false,
                updated_at: None,
                created_at: Utc::now().naive_utc(),
                language: LanguageAggregation {
                    id: 0,
                    code: String::from(create_dto.language),
                },
                versions: vec![ArticleVersionAggregation {
                    id: 0,
                    article_language_id: 0,
                    version: 1,
                    enabled: true,
                    updated_at: None,
                    created_at: Utc::now().naive_utc(),
                    content: VersionContentAggregation {
                        id: 0,
                        content: String::from(create_dto.content),
                    },
                }],
            }],
        }
    }
}
