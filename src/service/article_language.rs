use diesel::Connection;
use std::collections::HashMap;

use super::error::error_wrapper::ErrorWrapper;
use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::repository::connection;
use super::repository::entity::article_language::{ArticleLanguage, ArticleLanguageRepository};
use super::repository::entity::article_version::{ArticleVersion, ArticleVersionRepository};
use super::repository::entity::version_content::{
    ContentType, VersionContent, VersionContentRepository,
};

use super::article_version::ArticleVersionService;
use super::language::LanguageService;

use super::schema::article_language::{
    ArticleLanguageCreateDto, ArticleLanguageCreateRelationsDto, ArticleLanguagePatchDto,
};
use super::schema::article_version::{ArticleVersionCreateDto, LanguageSearchDto};
use super::schema::version_content::VersionContentDto;

use super::aggregation::article_language::ArticleLanguageAggregation;
use super::aggregation::article_version::ArticleVersionAggregation;
use super::aggregation::language::LanguageAggregation;

pub struct ArticleLanguageService {}

impl ArticleLanguageService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        query_options: QueryOptions,
    ) -> Result<ArticleLanguageAggregation, ErrorWrapper> {
        let (article_language, language) = match Self::get_one_with_language(
            connection,
            article_id,
            language_code,
            &query_options,
        )
        .await
        {
            Err(e) => return Err(e),
            Ok((article_language, language)) => (article_language, language),
        };

        Self::get_aggregation_with_relations(
            connection,
            article_id,
            &query_options,
            language,
            Some(article_language),
        )
        .await
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_id: i32,
        query_options: &QueryOptions,
    ) -> Vec<ArticleLanguageAggregation> {
        let article_languages =
            ArticleLanguageRepository::get_many(connection, vec![article_id], query_options).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let languages = LanguageService::get_aggregations(connection).await;
        let article_versions = match ArticleVersionService::get_aggregations(
            connection,
            LanguageSearchDto {
                article_languages_ids: Some(article_languages_ids),

                language_code: None,
                article_id: None,
                article_language: None,
            },
            query_options,
        )
        .await
        {
            Ok(article_versions) => article_versions,
            Err(_) => panic!(
                "{}",
                FmtError::FailedToFetch("article_versions").fmt().as_str()
            ),
        };

        ArticleLanguageAggregation::from_related_models(
            article_languages,
            article_versions,
            languages,
        )
    }

    pub async fn get_one_with_language(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        query_options: &QueryOptions,
    ) -> Result<(ArticleLanguage, LanguageAggregation), ErrorWrapper> {
        let language = match LanguageService::get_aggregation(connection, language_code).await {
            None => return FmtError::NotFound("language").error(),
            Some(language) => language,
        };

        let article_language = match ArticleLanguageRepository::get_one(
            connection,
            article_id,
            language.id,
            &query_options,
        )
        .await
        {
            None => return FmtError::NotFound("article_language").error(),
            Some(language) => language,
        };

        Ok((article_language, language))
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
    ) -> Result<ArticleLanguageAggregation, ErrorWrapper> {
        let language_code = String::from(&creation_dto.language_code);

        let language = match LanguageService::get_aggregation(connection, language_code).await {
            None => return FmtError::NotFound("language").error(),
            Some(language) => language,
        };

        match ArticleLanguageRepository::get_one(
            connection,
            creation_dto.article_id,
            language.id,
            &QueryOptions { is_actual: false },
        )
        .await
        {
            Some(_) => return FmtError::AlreadyExists("article_language").error(),
            _ => (),
        };

        let (article_language, version_content, article_version) =
            Self::create_relations_transaction(connection, creation_dto, language.id).await;

        let article_version_aggregations = ArticleVersionAggregation::from_related_models(
            vec![article_version],
            vec![version_content],
        );

        let article_language_aggregation = ArticleLanguageAggregation::from_related_models(
            vec![article_language],
            article_version_aggregations,
            vec![language],
        )
        .remove(0);

        Ok(article_language_aggregation)
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        language_code: String,
        article_id: i32,
        patch_dto: ArticleLanguagePatchDto,
    ) -> Result<ArticleLanguageAggregation, ErrorWrapper> {
        let language = match LanguageService::get_aggregation(connection, language_code).await {
            None => return FmtError::NotFound("language").error(),
            Some(language) => language,
        };

        let updated_count =
            ArticleLanguageRepository::patch(connection, language.id, article_id, patch_dto).await;

        if updated_count == 0 {
            return FmtError::NotFound("article_language").error();
        }

        Self::get_aggregation_with_relations(
            connection,
            article_id,
            &QueryOptions { is_actual: false },
            language,
            None,
        )
        .await
    }

    pub async fn get_aggregations_map(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
        query_options: &QueryOptions,
    ) -> HashMap<i32, Vec<ArticleLanguageAggregation>> {
        let article_languages =
            ArticleLanguageRepository::get_many(connection, article_ids, &query_options).await;

        let languages = LanguageService::get_aggregations(connection).await;

        let article_languages_ids: Vec<i32> = article_languages
            .iter()
            .map(|article_language| article_language.id)
            .collect();

        let article_versions = match ArticleVersionService::get_aggregations(
            connection,
            LanguageSearchDto {
                article_languages_ids: Some(article_languages_ids),

                language_code: None,
                article_id: None,
                article_language: None,
            },
            query_options,
        )
        .await
        {
            Ok(article_versions) => article_versions,
            Err(_) => panic!(
                "{}",
                FmtError::FailedToFetch("article_versions").fmt().as_str()
            ),
        };

        ArticleLanguageAggregation::get_aggregations_map(
            article_languages,
            article_versions,
            languages,
        )
    }

    async fn get_aggregation_with_relations(
        connection: &connection::PgConnection,
        article_id: i32,
        query_options: &QueryOptions,
        language: LanguageAggregation,
        article_language: Option<ArticleLanguage>,
    ) -> Result<ArticleLanguageAggregation, ErrorWrapper> {
        let article_language = match article_language {
            Some(article_language) => article_language,
            None => {
                match ArticleLanguageRepository::get_one(
                    connection,
                    article_id,
                    language.id,
                    &query_options,
                )
                .await
                {
                    None => return FmtError::NotFound("article_language").error(),
                    Some(language) => language,
                }
            }
        };

        let article_versions = match ArticleVersionService::get_aggregations(
            connection,
            LanguageSearchDto {
                article_languages_ids: Some(vec![article_language.id]),

                language_code: None,
                article_id: None,
                article_language: None,
            },
            query_options,
        )
        .await
        {
            Ok(article_versions) => article_versions,
            Err(_) => panic!(
                "{}",
                FmtError::FailedToFetch("article_versions").fmt().as_str()
            ),
        };

        let article_language_aggregation = ArticleLanguageAggregation::from_related_models(
            vec![article_language],
            article_versions,
            vec![language],
        )
        .remove(0);

        Ok(article_language_aggregation)
    }

    async fn create_relations_transaction(
        connection: &connection::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
        language_id: i32,
    ) -> (ArticleLanguage, VersionContent, ArticleVersion) {
        connection
            .run(move |connection| {
                return connection.transaction::<(ArticleLanguage, VersionContent, ArticleVersion), diesel::result::Error, _>(
                    |transaction_connection| {
                        Ok(Self::create_relations(
                            transaction_connection,
                            creation_dto,
                            language_id,
                        ))
                    },
                );
            })
            .await
            .expect(FmtError::FailedToInsert("article_language_relations").fmt().as_str())
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleLanguageCreateRelationsDto,
        language_id: i32,
    ) -> (ArticleLanguage, VersionContent, ArticleVersion) {
        let article_language = ArticleLanguageRepository::insert_raw(
            connection,
            ArticleLanguageCreateDto {
                name: creation_dto.name,
                article_id: creation_dto.article_id,
                language_id: language_id,
            },
        )
        .expect(FmtError::FailedToInsert("article_language").fmt().as_str());

        let version_content = VersionContentRepository::insert_raw(
            connection,
            VersionContentDto {
                content: creation_dto.content.as_bytes().to_vec(),
                content_type: ContentType::Full,
            },
        )
        .expect(FmtError::FailedToInsert("version_content").fmt().as_str());

        let article_version = ArticleVersionRepository::insert_raw(
            connection,
            ArticleVersionCreateDto {
                version: 1,
                article_language_id: article_language.id,
                content_id: version_content.id,
            },
        )
        .expect(FmtError::FailedToInsert("article_version").fmt().as_str());

        (article_language, version_content, article_version)
    }
}
