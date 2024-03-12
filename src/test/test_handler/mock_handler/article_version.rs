use chrono::Utc;

use super::aggregation::{
    article_version::ArticleVersionAggregation, version_content::VersionContentAggregation,
};

use super::dtm::article_version::request_body::ArticleVersionCreateRelationsBody;

pub struct ArticleVersionMockOptions {
    pub content: String,
    pub name: Option<String>,
    pub version: i32,
    pub enabled: bool,
}

impl ArticleVersionMockOptions {
    pub fn from_creation_dto(
        creation_dto: &ArticleVersionCreateRelationsBody,
        version: i32,
    ) -> Self {
        Self {
            version,

            enabled: true,
            content: creation_dto.content.clone(),
            name: creation_dto.name.clone(),
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
            name: String::from(""),
            version: mock_options.version,
            enabled: mock_options.enabled,
            updated_at: None,
            created_at: Utc::now().naive_utc(),
            content: VersionContentAggregation {
                id: 0,
                content: mock_options.content.clone(),
            },
            created_by: None,
        }
    }
}
