use super::repository::module::article_version::model::ArticleVersion;

use super::schema::article_version::ArticleVersionAggregation;

pub struct ArticleVersionMapper {}

impl ArticleVersionMapper {
    pub fn map_to_aggregations(
        article_versions: Vec<ArticleVersion>,
    ) -> Vec<ArticleVersionAggregation> {
        article_versions
            .into_iter()
            .map(|article_version| ArticleVersionAggregation {
                id: article_version.id,
                version: article_version.version,
                content: article_version.content,
                enabled: article_version.enabled,

                updated_at: article_version.updated_at,
                created_at: article_version.created_at,

                article_language_id: article_version.article_language_id,
            })
            .collect()
    }
}
