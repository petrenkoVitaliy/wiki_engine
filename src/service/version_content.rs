use super::repository::connection;
use super::repository::module::version_content::VersionContentRepository;

use super::schema::version_content::VersionContentAggregation;

use super::mapper::version_content::VersionContentMapper;

pub struct VersionContentService {}

impl VersionContentService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        id: i32,
    ) -> Option<VersionContentAggregation> {
        let version_content = match VersionContentRepository::get_one(connection, id).await {
            None => return None,
            Some(version_content) => version_content,
        };

        Some(VersionContentMapper::map_to_aggregation(version_content))
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        ids: Vec<i32>,
    ) -> Vec<VersionContentAggregation> {
        let version_contents = VersionContentRepository::get_many(connection, ids).await;

        VersionContentMapper::map_to_aggregations(version_contents)
    }
}
