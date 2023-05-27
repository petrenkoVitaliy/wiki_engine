use diesel::Connection;
use std::collections::HashMap;

use crate::schema::language::LanguageAggregation;

use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::repository::connection;
use super::repository::module::article_language::{model, ArticleLanguageRepository};
use super::repository::module::article_version::{model::ArticleVersion, ArticleVersionRepository};

use super::article_version::ArticleVersionService;
use super::language::LanguageService;

use super::schema::article_language::{
    ArticleLanguageAggregation, ArticleLanguageCreateDto, ArticleLanguageCreateRelationsDto,
    ArticleLanguagePatchDto,
};
use super::schema::article_version::ArticleVersionCreateDto;

use super::mapper::article_language::ArticleLanguageMapper;
use super::mapper::article_version::ArticleVersionMapper;

pub struct ArticleLanguageService {}

impl ArticleLanguageService {
    pub async fn get_one_by_language(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        query_options: QueryOptions,
    ) -> Option<model::ArticleLanguage> {
        let language = match LanguageService::get_aggregation(connection, language_code).await {
            None => return None,
            Some(language) => language,
        };

        match ArticleLanguageRepository::get_one(
            connection,
            article_id,
            language.id,
            &query_options,
        )
        .await
        {
            None => return None,
            Some(article_language) => Some(article_language),
        }
    }

    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        query_options: QueryOptions,
    ) -> Option<ArticleLanguageAggregation> {
        let language = match LanguageService::get_aggregation(connection, language_code).await {
            None => return None,
            Some(language) => language,
        };

        ArticleLanguageService::get_aggregation_with_language(
            connection,
            article_id,
            language,
            query_options,
        )
        .await
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
        query_options: QueryOptions,
    ) -> Vec<ArticleLanguageAggregation> {
        let article_languages = ArticleLanguageRepository::get_many(connection, article_ids).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let languages = LanguageService::get_aggregations(connection).await;
        let article_versions = ArticleVersionService::get_aggregations_by_languages(
            connection,
            article_languages_ids,
            query_options,
        )
        .await;

        ArticleLanguageMapper::map_to_aggregations(article_languages, article_versions, languages)
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
    ) -> ArticleLanguageAggregation {
        let language_code = String::from(&creation_dto.language_code);

        let language = LanguageService::get_aggregation(connection, language_code)
            .await
            .expect(FmtError::NotFound("language").fmt().as_str());

        match ArticleLanguageRepository::get_one(
            connection,
            creation_dto.article_id,
            language.id,
            &QueryOptions { is_actual: false },
        )
        .await
        {
            Some(_) => panic!(
                "{}",
                FmtError::AlreadyExists("article_language").fmt().as_str()
            ),
            _ => (),
        };

        let (article_language, article_version) =
            ArticleLanguageService::create_relations_transaction(
                connection,
                creation_dto,
                language.id,
            )
            .await;

        let article_version_aggregations =
            ArticleVersionMapper::map_to_aggregations(vec![article_version]);

        let article_language_aggregation = ArticleLanguageMapper::map_to_aggregations(
            vec![article_language],
            article_version_aggregations,
            vec![language],
        )
        .remove(0);

        article_language_aggregation
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        language_code: String,
        article_id: i32,
        patch_dto: ArticleLanguagePatchDto,
    ) -> Option<ArticleLanguageAggregation> {
        let language: LanguageAggregation =
            match LanguageService::get_aggregation(connection, language_code).await {
                None => return None,
                Some(language) => language,
            };

        let updated_count =
            ArticleLanguageRepository::patch(connection, language.id, article_id, patch_dto).await;

        if updated_count == 0 {
            return None;
        }

        ArticleLanguageService::get_aggregation_with_language(
            connection,
            article_id,
            language,
            QueryOptions { is_actual: false },
        )
        .await
    }

    pub async fn get_aggregations_map(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
    ) -> HashMap<i32, Vec<ArticleLanguageAggregation>> {
        let query_options = QueryOptions { is_actual: false };

        let article_languages = ArticleLanguageRepository::get_many(connection, article_ids).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let languages = LanguageService::get_aggregations(connection).await;
        let article_versions = ArticleVersionService::get_aggregations_by_languages(
            connection,
            article_languages_ids,
            query_options,
        )
        .await;

        ArticleLanguageMapper::map_to_aggregations_map(
            article_languages,
            article_versions,
            languages,
        )
    }

    async fn get_aggregation_with_language(
        connection: &connection::PgConnection,
        article_id: i32,
        language: LanguageAggregation,
        query_options: QueryOptions,
    ) -> Option<ArticleLanguageAggregation> {
        let article_language = match ArticleLanguageRepository::get_one(
            connection,
            article_id,
            language.id,
            &query_options,
        )
        .await
        {
            None => return None,
            Some(article) => article,
        };

        let article_versions = ArticleVersionService::get_aggregations_by_languages(
            connection,
            vec![article_language.id],
            query_options,
        )
        .await;

        let article_language_aggregation = ArticleLanguageMapper::map_to_aggregations(
            vec![article_language],
            article_versions,
            vec![language],
        )
        .remove(0);

        Some(article_language_aggregation)
    }

    async fn create_relations_transaction(
        connection: &connection::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
        language_id: i32,
    ) -> (model::ArticleLanguage, ArticleVersion) {
        connection
            .run(move |connection| {
                return connection.transaction::<(model::ArticleLanguage, ArticleVersion), diesel::result::Error, _>(
                    |transaction_connection| {
                        Ok(ArticleLanguageService::create_relations(
                            transaction_connection,
                            creation_dto,
                            language_id,
                        ))
                    },
                );
            })
            .await
            .expect("failed to create article_language relations")
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
        language_id: i32,
    ) -> (model::ArticleLanguage, ArticleVersion) {
        let article_language = ArticleLanguageRepository::insert_raw(
            connection,
            ArticleLanguageCreateDto {
                name: creation_dto.name,
                article_id: creation_dto.article_id,
                language_id: language_id,
            },
        )
        .expect(FmtError::FailedToInsert("article_language").fmt().as_str());

        let article_version = ArticleVersionRepository::insert_raw(
            connection,
            ArticleVersionCreateDto {
                version: 1,
                article_language_id: article_language.id,
                content: creation_dto.content,
            },
        )
        .expect(FmtError::FailedToInsert("article_version").fmt().as_str());

        (article_language, article_version)
    }
}
