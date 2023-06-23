use chrono::Utc;

use super::aggregation::{
    article_language::ArticleLanguageAggregation, article_version::ArticleVersionAggregation,
    language::LanguageAggregation, version_content::VersionContentAggregation,
};

use super::schema::article_language::ArticleLanguageCreateBody;

pub struct ArticleLanguageMockOptions {
    pub content: String,
    pub name: String,
    pub language: String,
}

impl ArticleLanguageMockOptions {
    pub fn from_creation_dto(creation_dto: ArticleLanguageCreateBody, language: String) -> Self {
        Self {
            language,
            content: String::from(creation_dto.content),
            name: String::from(creation_dto.name),
        }
    }
}

pub struct ArticleLanguageMockHandler;
impl ArticleLanguageMockHandler {
    pub fn get_article_language_aggregation(
        mock_options: ArticleLanguageMockOptions,
    ) -> ArticleLanguageAggregation {
        ArticleLanguageAggregation {
            id: 0,
            name: String::from(mock_options.name),
            enabled: true,
            archived: false,
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            language: LanguageAggregation {
                id: 0,
                code: String::from(mock_options.language),
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
                    content: String::from(mock_options.content),
                },
            }],
        }
    }
}
