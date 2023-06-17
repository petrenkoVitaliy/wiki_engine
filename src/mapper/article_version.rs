use std::collections::HashMap;

use super::error::formatted_error::FmtError;

use super::repository::module::article_version::model::ArticleVersion;
use super::repository::module::version_content::model::VersionContent;

use super::schema::article_version::ArticleVersionAggregation;
use super::schema::version_content::VersionContentAggregation;

use super::version_content::VersionContentMapper;

pub struct ArticleVersionMapper {}

impl ArticleVersionMapper {
    pub fn map_to_aggregations_with_content(
        article_versions: Vec<ArticleVersion>,
        version_content: Vec<VersionContentAggregation>,
    ) -> Vec<ArticleVersionAggregation> {
        let mut content_map = Self::get_aggregation_content_map(version_content);

        article_versions
            .into_iter()
            .map(move |article_version| {
                let content_version_aggregation = content_map
                    .remove(&article_version.content_id)
                    .expect(FmtError::NotFound("article_versions").fmt().as_str());

                return ArticleVersionAggregation {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: content_version_aggregation,
                };
            })
            .collect()
    }

    pub fn map_to_aggregations(
        article_versions: Vec<ArticleVersion>,
        version_content: Vec<VersionContent>,
    ) -> Vec<ArticleVersionAggregation> {
        let mut content_map = Self::get_content_map(version_content);

        article_versions
            .into_iter()
            .map(move |article_version| {
                let content_version = content_map
                    .remove(&article_version.content_id)
                    .expect(FmtError::NotFound("article_versions").fmt().as_str());

                return ArticleVersionAggregation {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentMapper::map_to_aggregation(content_version, None),
                };
            })
            .collect()
    }

    pub fn map_to_aggregations_with_content_map(
        article_versions_with_contents: Vec<(ArticleVersion, VersionContent)>,
        contents_map: HashMap<i32, String>,
    ) -> Vec<ArticleVersionAggregation> {
        article_versions_with_contents
            .into_iter()
            .map(move |(article_version, version_content)| {
                return ArticleVersionAggregation {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentMapper::map_to_aggregation(
                        version_content,
                        Some(&contents_map),
                    ),
                };
            })
            .collect()
    }

    fn get_content_map(version_content: Vec<VersionContent>) -> HashMap<i32, VersionContent> {
        version_content
            .into_iter()
            .fold(HashMap::new(), |mut acc, version_content| {
                acc.insert(version_content.id, version_content);

                acc
            })
    }

    fn get_aggregation_content_map(
        version_content_aggregations: Vec<VersionContentAggregation>,
    ) -> HashMap<i32, VersionContentAggregation> {
        version_content_aggregations
            .into_iter()
            .fold(HashMap::new(), |mut acc, version_content| {
                acc.insert(version_content.id, version_content);

                acc
            })
    }
}
