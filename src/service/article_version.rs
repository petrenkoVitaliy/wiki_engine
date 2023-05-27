use super::repository::connection;
use super::repository::module::article_version::ArticleVersionRepository;

use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::article_language::ArticleLanguageService;

use super::schema::article_version::{
    ArticleVersionAggregation, ArticleVersionCreateBody, ArticleVersionCreateDto,
    ArticleVersionPatchBody, ArticleVersionPatchDto, ArticleVersionSearchDto,
    ArticleVersionsSearchDto,
};

use super::mapper::article_version::ArticleVersionMapper;

pub struct ArticleVersionService {}

impl ArticleVersionService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        id: i32,
        article_id: i32,
        language_code: String,
        query_options: QueryOptions,
    ) -> Option<ArticleVersionAggregation> {
        let article_language = match ArticleLanguageService::get_one_by_language(
            connection,
            article_id,
            language_code,
            QueryOptions { is_actual: true },
        )
        .await
        {
            None => return None,
            Some(article_language) => article_language,
        };

        let article_version = match ArticleVersionRepository::get_one(
            connection,
            ArticleVersionSearchDto {
                id: Some(id),
                article_languages_ids: Some(vec![article_language.id]),
            },
            &query_options,
        )
        .await
        {
            None => return None,
            Some(article_version) => article_version,
        };

        let article_version_aggregation =
            ArticleVersionMapper::map_to_aggregations(vec![article_version]).remove(0);

        return Some(article_version_aggregation);
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        query_options: QueryOptions,
    ) -> Vec<ArticleVersionAggregation> {
        let article_language = match ArticleLanguageService::get_one_by_language(
            connection,
            article_id,
            language_code,
            QueryOptions { is_actual: true },
        )
        .await
        {
            None => panic!("{}", FmtError::NotFound("article_language").fmt().as_str()),
            Some(article_language) => article_language,
        };

        let article_versions: Vec<
            crate::repository::module::article_version::model::ArticleVersion,
        > = ArticleVersionRepository::get_many(
            connection,
            ArticleVersionsSearchDto {
                ids: None,
                article_languages_ids: Some(vec![article_language.id]),
            },
            &query_options,
        )
        .await;

        ArticleVersionMapper::map_to_aggregations(article_versions)
    }

    pub async fn get_aggregations_by_languages(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
        query_options: QueryOptions,
    ) -> Vec<ArticleVersionAggregation> {
        let article_versions = ArticleVersionRepository::get_many(
            connection,
            ArticleVersionsSearchDto {
                ids: None,
                article_languages_ids: Some(article_languages_ids),
            },
            &query_options,
        )
        .await;

        ArticleVersionMapper::map_to_aggregations(article_versions)
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        creation_body: ArticleVersionCreateBody,
    ) -> Option<ArticleVersionAggregation> {
        let article_language = match ArticleLanguageService::get_one_by_language(
            connection,
            article_id,
            language_code,
            QueryOptions { is_actual: true },
        )
        .await
        {
            None => return None,
            Some(article_language) => article_language,
        };

        let article_versions_count =
            ArticleVersionRepository::get_count(connection, article_language.id).await;

        let article_version = ArticleVersionRepository::insert(
            connection,
            ArticleVersionCreateDto {
                version: article_versions_count + 1,
                content: creation_body.content,
                article_language_id: article_language.id,
            },
        )
        .await;

        Some(ArticleVersionMapper::map_to_aggregations(vec![article_version]).remove(0))
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        id: i32,
        article_id: i32,
        language_code: String,
        patch_body: ArticleVersionPatchBody,
    ) -> Option<ArticleVersionAggregation> {
        let article_language = match ArticleLanguageService::get_one_by_language(
            connection,
            article_id,
            language_code,
            QueryOptions { is_actual: true },
        )
        .await
        {
            None => return None,
            Some(article_language) => article_language,
        };

        let article_version = ArticleVersionRepository::patch(
            connection,
            id,
            article_language.id,
            ArticleVersionPatchDto {
                enabled: patch_body.enabled,
            },
        )
        .await;

        Some(ArticleVersionMapper::map_to_aggregations(vec![article_version]).remove(0))
    }
}
