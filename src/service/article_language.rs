use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::repository::connection;
use super::repository::module::article_language::{model, ArticleLanguageRepository};

use super::schema::article_language::ArticleLanguageAggregation;
use super::schema::article_version::ArticleVersionAggregation;
use super::schema::language::LanguageAggregation;

use super::article_version::ArticleVersionService;
use super::language::LanguageService;

pub struct ArticleLanguageService {}

impl ArticleLanguageService {
    async fn get_many(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
    ) -> Vec<model::ArticleLanguage> {
        connection::wrap_db(
            &connection,
            ArticleLanguageRepository::get_many_by_article,
            article_ids,
            "failed to fetch article_language",
        )
        .await
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
    ) -> Vec<ArticleLanguageAggregation> {
        let article_languages = ArticleLanguageService::get_many(connection, article_ids).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let languages = LanguageService::get_aggregations(connection).await;
        let article_versions =
            ArticleVersionService::get_aggregations(connection, article_languages_ids).await;

        ArticleLanguageService::map_to_aggregations(article_languages, article_versions, languages)
    }

    pub async fn get_aggregations_map(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
    ) -> HashMap<i32, Vec<ArticleLanguageAggregation>> {
        let article_languages = ArticleLanguageService::get_many(connection, article_ids).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let languages = LanguageService::get_aggregations(connection).await;
        let article_versions =
            ArticleVersionService::get_aggregations(connection, article_languages_ids).await;

        ArticleLanguageService::map_to_aggregations_map(
            article_languages,
            article_versions,
            languages,
        )
    }

    pub fn map_to_aggregations(
        article_languages: Vec<model::ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> Vec<ArticleLanguageAggregation> {
        let mut article_versions_map =
            ArticleLanguageService::get_article_versions_map(article_versions);

        let languages_map = ArticleLanguageService::get_languages_map(languages);

        article_languages
            .into_iter()
            .map(|article_language| {
                ArticleLanguageService::map_to_aggregation(
                    article_language,
                    &mut article_versions_map,
                    &languages_map,
                )
            })
            .collect()
    }

    //  ¯\_(ツ)_/¯
    fn map_to_aggregations_map(
        article_languages: Vec<model::ArticleLanguage>,
        article_versions: Vec<ArticleVersionAggregation>,
        languages: Vec<LanguageAggregation>,
    ) -> HashMap<i32, Vec<ArticleLanguageAggregation>> {
        let mut article_versions_map =
            ArticleLanguageService::get_article_versions_map(article_versions);

        let languages_map = ArticleLanguageService::get_languages_map(languages);

        article_languages
            .into_iter()
            .fold(HashMap::new(), |mut acc, article_language| {
                let article_id = article_language.article_id;

                let article_language_aggregation = ArticleLanguageService::map_to_aggregation(
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
        article_language: model::ArticleLanguage,
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
                .expect("language wasn't found")
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
