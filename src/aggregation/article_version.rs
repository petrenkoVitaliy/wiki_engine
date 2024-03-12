use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use std::collections::HashMap;

use super::error::FmtError;
use super::mapper::ValuesMapper;

use super::repository::entity::{
    article_version::ArticleVersion, auth::UserAccount, version_content::VersionContent,
};

use super::user_account::UserAccountPartialAggregation;
use super::version_content::VersionContentAggregation;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ArticleVersionAggregation {
    pub id: i32,
    pub version: i32,
    pub name: String,
    pub enabled: bool,

    pub content: VersionContentAggregation,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub created_by: Option<UserAccountPartialAggregation>,

    pub article_language_id: i32,
}

impl ArticleVersionAggregation {
    pub fn from_related_models(
        article_versions: Vec<ArticleVersion>,
        version_contents: Vec<VersionContent>,
    ) -> Vec<Self> {
        let mut content_map = ValuesMapper::vector_to_hashmap(version_contents, |ver| ver.id);

        article_versions
            .into_iter()
            .map(move |article_version| {
                let content_version = content_map
                    .remove(&article_version.content_id)
                    .expect(&FmtError::NotFound("article_versions").fmt());

                return Self {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,
                    name: article_version.name,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentAggregation::from_model(content_version, None),

                    created_by: None,
                };
            })
            .collect()
    }

    pub fn from_content_map(
        article_versions_relations: Vec<(ArticleVersion, VersionContent, UserAccount)>,
        contents_map: HashMap<i32, String>,
    ) -> Vec<Self> {
        article_versions_relations
            .into_iter()
            .map(move |(article_version, version_content, user_account)| {
                return Self {
                    id: article_version.id,
                    version: article_version.version,
                    enabled: article_version.enabled,
                    name: article_version.name,

                    updated_at: article_version.updated_at,
                    created_at: article_version.created_at,

                    article_language_id: article_version.article_language_id,
                    content: VersionContentAggregation::from_model(
                        version_content,
                        Some(&contents_map),
                    ),

                    created_by: Some(UserAccountPartialAggregation::from_model(user_account)),
                };
            })
            .collect()
    }
}
