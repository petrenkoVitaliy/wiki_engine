use chrono::Utc;

use super::aggregation::{
    article_version::ArticleVersionAggregation, version_content::VersionContentAggregation,
};

use super::schema::article_version::ArticleVersionCreateBody;

pub struct ArticleVersionMockOptions {
    pub content: String,
    pub version: i32,
    pub enabled: bool,
}

impl ArticleVersionMockOptions {
    pub fn from_creation_dto(creation_dto: &ArticleVersionCreateBody, version: i32) -> Self {
        Self {
            version,

            enabled: true,
            content: creation_dto.content.clone(),
        }
    }
}

pub struct ArticleVersionMockHandler;
impl ArticleVersionMockHandler {
    pub fn get_article_version_aggregation(
        mock_options: &ArticleVersionMockOptions,
    ) -> ArticleVersionAggregation {
        ArticleVersionAggregation {
            id: 0,
            article_language_id: 0,
            version: mock_options.version,
            enabled: mock_options.enabled,
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            content: VersionContentAggregation {
                id: 0,
                content: mock_options.content.clone(),
            },
        }
    }
}
