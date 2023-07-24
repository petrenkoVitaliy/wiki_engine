use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use std::collections::HashMap;

use super::repository::entity::{
    article::{Article, ArticleType},
    article_language::ArticleLanguage,
    article_version::ArticleVersion,
    language::Language,
    version_content::VersionContent,
};

use super::article_language::ArticleLanguageAggregation;
use super::article_version::ArticleVersionAggregation;
use super::language::LanguageAggregation;

#[derive(Serialize, JsonSchema, Debug, Deserialize)]
pub struct ArticleAggregation {
    pub id: i32,
    pub enabled: bool,
    pub archived: bool,
    pub article_type: ArticleType,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub languages: Vec<ArticleLanguageAggregation>,
}

impl ArticleAggregation {
    pub fn from_model(
        article: Article,
        article_language_aggregations: Vec<ArticleLanguageAggregation>,
    ) -> Self {
        Self {
            id: article.id,
            enabled: article.enabled,
            article_type: article.article_type,
            archived: article.archived,
            updated_at: article.updated_at,
            created_at: article.created_at,

            languages: article_language_aggregations,
        }
    }

    pub fn from_languages_map(
        articles: Vec<Article>,
        mut languages_aggregations_map: HashMap<i32, Vec<ArticleLanguageAggregation>>,
    ) -> Vec<Self> {
        articles
            .into_iter()
            .map(|article| {
                let article_languages_aggregations = languages_aggregations_map
                    .remove(&article.id)
                    .unwrap_or(vec![]);

                Self::from_model(article, article_languages_aggregations)
            })
            .collect()
    }

    pub fn from_related_models(
        article: Article,
        article_language: ArticleLanguage,
        article_version: ArticleVersion,
        version_content: VersionContent,
        language: Language,
    ) -> Self {
        let article_versions_aggregations = ArticleVersionAggregation::from_related_models(
            vec![article_version],
            vec![version_content],
        );

        let languages_aggregation = LanguageAggregation::from_model(language);
        let article_language_aggregations = ArticleLanguageAggregation::from_related_models(
            vec![article_language],
            article_versions_aggregations,
            vec![languages_aggregation],
        );

        Self::from_model(article, article_language_aggregations)
    }
}
