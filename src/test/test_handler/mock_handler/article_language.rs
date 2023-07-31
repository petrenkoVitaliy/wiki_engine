use chrono::Utc;

use super::aggregation::{
    article_language::ArticleLanguageAggregation, language::LanguageAggregation,
};

use super::dtm::article_language::request_body::ArticleLanguageCreateRelationsBody;

use super::article_version::{ArticleVersionMockHandler, ArticleVersionMockOptions};

pub struct ArticleLanguageMockOptions {
    pub content: String,
    pub name: String,
    pub language: String,
    pub enabled: bool,
    pub archived: bool,
}

impl ArticleLanguageMockOptions {
    pub fn from_creation_dto(
        creation_dto: &ArticleLanguageCreateRelationsBody,
        language: &String,
    ) -> Self {
        Self {
            enabled: true,
            archived: false,

            language: String::from(language),
            content: creation_dto.content.clone(),
            name: creation_dto.name.clone(),
        }
    }
}

pub struct ArticleLanguageMockHandler;
impl ArticleLanguageMockHandler {
    pub fn get_article_language_aggregation(
        mock_options: &ArticleLanguageMockOptions,
    ) -> ArticleLanguageAggregation {
        ArticleLanguageAggregation {
            id: 0,
            name: mock_options.name.clone(),
            enabled: mock_options.enabled,
            archived: mock_options.archived,
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            language: LanguageAggregation {
                id: 0,
                code: mock_options.language.clone(),
            },
            version: ArticleVersionMockHandler::get_article_version_aggregation(
                &ArticleVersionMockOptions {
                    enabled: true,
                    version: 1,
                    content: mock_options.content.clone(),
                },
            ),
        }
    }
}
