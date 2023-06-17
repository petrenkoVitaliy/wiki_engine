use std::collections::HashMap;

use super::repository::connection;
use super::repository::models::article_version::model::ArticleVersion;
use super::repository::models::version_content::{model, VersionContentRepository};

use super::schema::version_content::ContentType;

use super::aggregation::version_content::VersionContentAggregation;

use super::diff_handler::diff_handler::DiffHandler;

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

        Some(VersionContentAggregation::from_model(version_content, None))
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        ids: Vec<i32>,
    ) -> Vec<VersionContentAggregation> {
        let version_contents = VersionContentRepository::get_many(connection, ids).await;

        VersionContentAggregation::from_model_list(version_contents)
    }

    pub fn get_contents_map_by_ids(
        article_versions_with_contents: &Vec<(ArticleVersion, model::VersionContent)>,
    ) -> Option<HashMap<i32, String>> {
        let full_content = match article_versions_with_contents.last() {
            None => return None,
            Some((article_version, version_content)) => {
                if !matches!(version_content.content_type, ContentType::Full) {
                    return None;
                }

                (article_version, version_content)
            }
        };

        let (content_map, _) = article_versions_with_contents
            [0..article_versions_with_contents.len() - 1]
            .into_iter()
            .rev()
            .map(|(_, version_content)| version_content)
            .fold(
                (
                    HashMap::new(),
                    DiffHandler::get_string_from_bytes(&full_content.1.content),
                ),
                |(mut content_map, previous_full_content), version_content| {
                    let content = DiffHandler::get_patch(
                        &version_content.content,
                        version_content.content_length,
                        previous_full_content,
                    );

                    let current_content = String::from(&content);

                    content_map.insert(version_content.id, current_content);

                    return (content_map, content);
                },
            );

        Some(content_map)
    }
}
