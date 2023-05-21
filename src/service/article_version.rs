use super::repository::connection;
use super::repository::module::article_version::{model, ArticleVersionRepository};

use super::schema::article_version::ArticleVersionAggregation;

pub struct ArticleVersionService {}

impl ArticleVersionService {
    async fn get_many(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Vec<model::ArticleVersion> {
        connection::wrap_db(
            connection,
            ArticleVersionRepository::get_many_by_languages,
            article_languages_ids,
            "failed to fetch article versions",
        )
        .await
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Vec<ArticleVersionAggregation> {
        let article_versions =
            ArticleVersionService::get_many(connection, article_languages_ids).await;

        ArticleVersionService::map_to_aggregations(article_versions)
    }

    pub fn map_to_aggregations(
        article_versions: Vec<model::ArticleVersion>,
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
