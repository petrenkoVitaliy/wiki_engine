use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::error::formatted_error::FmtError;

use super::repository::module::article_language::model::ArticleLanguage;

use super::schema::article_language::ArticleLanguageAggregation;
use super::schema::article_version::ArticleVersionAggregation;
use super::schema::language::LanguageAggregation;

pub struct ArticleLanguageMapper {}

impl ArticleLanguageMapper {
    pub fn map_to_aggregations(
        article_languages: Vec<ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> Vec<ArticleLanguageAggregation> {
        let mut article_versions_map: HashMap<i32, Vec<ArticleVersionAggregation>> =
            ArticleLanguageMapper::get_article_versions_map(article_versions);

        let languages_map = ArticleLanguageMapper::get_languages_map(languages);

        article_languages
            .into_iter()
            .map(|article_language| {
                ArticleLanguageMapper::map_to_aggregation(
                    article_language,
                    &mut article_versions_map,
                    &languages_map,
                )
            })
            .collect()
    }

    //  ¯\_(ツ)_/¯
    pub fn map_to_aggregations_map(
        article_languages: Vec<ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> HashMap<i32, Vec<ArticleLanguageAggregation>> {
        let mut article_versions_map =
            ArticleLanguageMapper::get_article_versions_map(article_versions);

        let languages_map = ArticleLanguageMapper::get_languages_map(languages);

        article_languages
            .into_iter()
            .fold(HashMap::new(), |mut acc, article_language| {
                let article_id = article_language.article_id;

                let article_language_aggregation = ArticleLanguageMapper::map_to_aggregation(
                    article_language,
                    &mut article_versions_map,
                    &languages_map,
                );

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

    fn map_to_aggregation(
        article_language: ArticleLanguage,
        article_versions_map: &mut HashMap<i32, Vec<ArticleVersionAggregation>>,
        languages_map: &HashMap<i32, LanguageAggregation>,
    ) -> ArticleLanguageAggregation {
        ArticleLanguageAggregation {
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

    fn get_languages_map(languages: Vec<LanguageAggregation>) -> HashMap<i32, LanguageAggregation> {
        languages
            .into_iter()
            .fold(HashMap::new(), |mut acc, language| {
                acc.insert(language.id, language);

                acc
            })
    }
}
