use chrono::Utc;

use super::{
    ArticleAggregation, ArticleCreateRelationsDto, ArticleLanguageAggregation,
    ArticleVersionAggregation, LanguageAggregation, VersionContentAggregation,
};

pub struct ArticleExpectedMock {}

impl ArticleExpectedMock {
    pub fn get_article_aggregation(create_dto: ArticleCreateRelationsDto) -> ArticleAggregation {
        ArticleAggregation {
            id: 0,
            enabled: true,
            archived: false,
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
