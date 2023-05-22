use super::repository::connection;
use super::repository::module::article_version::ArticleVersionRepository;

use super::schema::article_version::ArticleVersionAggregation;

use super::mapper::article_version::ArticleVersionMapper;

pub struct ArticleVersionService {}

impl ArticleVersionService {
    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Vec<ArticleVersionAggregation> {
        let article_versions =
            ArticleVersionRepository::get_many(connection, article_languages_ids).await;

        ArticleVersionMapper::map_to_aggregations(article_versions)
    }
}
