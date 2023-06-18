use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use std::collections::{hash_map::Entry, HashMap};

use super::error::formatted_error::FmtError;
use super::mapper::values_mapper::ValuesMapper;

use super::repository::entity::article_language::ArticleLanguage;

use super::article_version::ArticleVersionAggregation;
use super::language::LanguageAggregation;

#[derive(Serialize, JsonSchema, Deserialize, Debug)]
pub struct ArticleLanguageAggregation {
    pub id: i32,
    pub name: String,

    pub enabled: bool,
    pub archived: bool,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub language: LanguageAggregation,
    pub versions: Vec<ArticleVersionAggregation>,
}

impl ArticleLanguageAggregation {
    pub fn from_related_models(
        article_languages: Vec<ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> Vec<Self> {
        let mut article_versions_map: HashMap<i32, Vec<ArticleVersionAggregation>> =
            Self::get_article_versions_map(article_versions);

        let languages_map = ValuesMapper::vector_to_hashmap(languages, |lang| lang.id);

        article_languages
            .into_iter()
            .map(|article_language| {
                Self::from_model(article_language, &mut article_versions_map, &languages_map)
            })
            .collect()
    }

    pub fn get_aggregations_map(
        article_languages: Vec<ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> HashMap<i32, Vec<Self>> {
        let mut article_versions_map = Self::get_article_versions_map(article_versions);

        let languages_map = ValuesMapper::vector_to_hashmap(languages, |lang| lang.id);

        article_languages
            .into_iter()
            .fold(HashMap::new(), |mut acc, article_language| {
                let article_id = article_language.article_id;

                let article_language_aggregation =
                    Self::from_model(article_language, &mut article_versions_map, &languages_map);

                match acc.entry(article_id) {
                    Entry::Vacant(acc) => {
                        acc.insert(vec![article_language_aggregation]);
                    }
                    Entry::Occupied(mut acc) => {
                        acc.get_mut().push(article_language_aggregation);
                    }
                };

                acc
            })
    }

    fn from_model(
        article_language: ArticleLanguage,
        article_versions_map: &mut HashMap<i32, Vec<ArticleVersionAggregation>>,
        languages_map: &HashMap<i32, LanguageAggregation>,
    ) -> Self {
        Self {
            id: article_language.id,
            name: article_language.name,
            enabled: article_language.enabled,
            archived: article_language.archived,

            updated_at: article_language.updated_at,
            created_at: article_language.created_at,

            versions: article_versions_map
                .remove(&article_language.id)
                .unwrap_or(vec![]),

            language: languages_map
                .get(&article_language.language_id)
                .expect(FmtError::NotFound("language").fmt().as_str())
                .clone(),
        }
    }

    fn get_article_versions_map(
        article_versions: Vec<ArticleVersionAggregation>,
    ) -> HashMap<i32, Vec<ArticleVersionAggregation>> {
        article_versions
            .into_iter()
            .fold(HashMap::new(), |mut acc, article_version| {
                match acc.entry(article_version.article_language_id) {
                    Entry::Vacant(acc) => {
                        acc.insert(vec![article_version]);
                    }
                    Entry::Occupied(mut acc) => {
                        acc.get_mut().push(article_version);
                    }
                };

                acc
            })
    }
}
