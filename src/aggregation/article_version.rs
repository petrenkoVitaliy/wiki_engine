use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::collections::HashMap;

use super::error::formatted_error::FmtError;
use super::mapper::values_mapper::ValuesMapper;

use super::repository::models::article_version::model::ArticleVersion;
use super::repository::models::version_content::model::VersionContent;

use super::version_content::VersionContentAggregation;

#[derive(Serialize, JsonSchema)]
pub struct ArticleVersionAggregation {
    pub id: i32,
    pub version: i32,
    pub enabled: bool,

    pub content: VersionContentAggregation,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub article_language_id: i32,
}

impl ArticleVersionAggregation {
    pub fn from_model_list_with_content(
        article_versions: Vec<ArticleVersion>,
        version_content: Vec<VersionContentAggregation>,
    ) -> Vec<Self> {
        let mut content_map = ValuesMapper::vector_to_hashmap(version_content, |ver| ver.id);

        article_versions
            .into_iter()
            .map(move |article_version| {
                let content_version_aggregation = content_map
                    .remove(&article_version.content_id)
                    .expect(FmtError::NotFound("article_versions").fmt().as_str());

                return Self {
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

    pub fn from_related_models(
        article_versions: Vec<ArticleVersion>,
        version_content: Vec<VersionContent>,
    ) -> Vec<Self> {
        let mut content_map = ValuesMapper::vector_to_hashmap(version_content, |ver| ver.id);

        article_versions
            .into_iter()
            .map(move |article_version| {
                let content_version = content_map
                    .remove(&article_version.content_id)
                    .expect(FmtError::NotFound("article_versions").fmt().as_str());

                return Self {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentAggregation::from_model(content_version, None),
                };
            })
            .collect()
    }

    pub fn from_content_map(
        article_versions_with_contents: Vec<(ArticleVersion, VersionContent)>,
        contents_map: HashMap<i32, String>,
    ) -> Vec<Self> {
        article_versions_with_contents
            .into_iter()
            .map(move |(article_version, version_content)| {
                return Self {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentAggregation::from_model(
                        version_content,
                        Some(&contents_map),
                    ),
                };
            })
            .collect()
    }
}
