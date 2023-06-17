use diesel::Connection;

use super::repository::connection;
use super::repository::models::article_version::{model, ArticleVersionRepository};
use super::repository::models::version_content::{
    model::{ContentType, VersionContent},
    VersionContentRepository,
};

use super::diff_handler::diff_handler::DiffHandler;
use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::article_language::ArticleLanguageService;
use super::version_content::VersionContentService;

use super::schema::article_version::{
    ArticleVersionCreateBody, ArticleVersionCreateDto, ArticleVersionPatchBody,
    ArticleVersionPatchDto, ArticleVersionSearchDto, ArticleVersionsJoinSearchDto,
    ArticleVersionsSearchDto,
};
use super::schema::version_content::VersionContentDto;

use super::aggregation::article_version::ArticleVersionAggregation;

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

        let version_content =
            match VersionContentService::get_aggregation(connection, article_version.content_id)
                .await
            {
                None => return None,
                Some(version_content) => version_content,
            };

        let article_version_aggregation = ArticleVersionAggregation::from_model_list_with_content(
            vec![article_version],
            vec![version_content],
        )
        .remove(0);

        return Some(article_version_aggregation);
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        // TODO is active
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

        let article_versions_contents = ArticleVersionRepository::get_many_with_content(
            connection,
            ArticleVersionsJoinSearchDto {
                version_gt: None,
                article_languages_ids: Some(vec![article_language.id]),
            },
        )
        .await;

        let content_map =
            match VersionContentService::get_contents_map_by_ids(&article_versions_contents) {
                None => panic!(
                    "{}",
                    FmtError::FailedToProcess("content_map").fmt().as_str()
                ),
                Some(map) => map,
            };

        let article_versions_aggregations =
            ArticleVersionAggregation::from_content_map(article_versions_contents, content_map);

        return article_versions_aggregations
            .into_iter()
            .filter(move |aggregation| {
                if !query_options.is_actual {
                    return true;
                }

                return aggregation.enabled;
            })
            .collect();
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

        let article_versions_ids: Vec<i32> = article_versions
            .iter()
            .map(|article_version| article_version.id)
            .collect();

        let version_content =
            VersionContentService::get_aggregations(connection, article_versions_ids).await;

        ArticleVersionAggregation::from_model_list_with_content(article_versions, version_content)
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

        let (article_version, version_content) = Self::create_relations_transaction(
            connection,
            creation_body,
            article_language.id,
            article_versions_count,
        )
        .await;

        Some(
            ArticleVersionAggregation::from_related_models(
                vec![article_version],
                vec![version_content],
            )
            .remove(0),
        )
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

        let version_content =
            match VersionContentService::get_aggregation(connection, article_version.content_id)
                .await
            {
                None => return None,
                Some(version_content) => version_content,
            };

        Some(
            ArticleVersionAggregation::from_model_list_with_content(
                vec![article_version],
                vec![version_content],
            )
            .remove(0),
        )
    }

    async fn create_relations_transaction(
        connection: &connection::PgConnection,
        creation_body: ArticleVersionCreateBody,
        article_language_id: i32,
        article_versions_count: i32,
    ) -> (model::ArticleVersion, VersionContent) {
        connection
            .run(move |connection| {
                return connection.transaction::<(model::ArticleVersion, VersionContent), diesel::result::Error, _>(
                    |transaction_connection| {
                        Ok(Self::create_relations(
                            transaction_connection,
                            creation_body,
                            article_language_id,
                            article_versions_count
                        ))
                    },
                );
            })
            .await
            .expect("failed to create article_version relations")
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_body: ArticleVersionCreateBody,
        article_language_id: i32,
        article_versions_count: i32,
    ) -> (model::ArticleVersion, VersionContent) {
        if article_versions_count > 0 {
            let article_version =
                ArticleVersionRepository::get_by_version_raw(connection, article_versions_count)
                    .expect(FmtError::FailedToFetch("article_version").fmt().as_str())
                    .expect(FmtError::NotFound("article_version").fmt().as_str());

            let version_content =
                VersionContentRepository::get_one_raw(connection, article_version.content_id)
                    .expect(FmtError::FailedToFetch("version_content").fmt().as_str())
                    .expect(FmtError::NotFound("version_content").fmt().as_str());

            let content_delta =
                DiffHandler::get_delta(&creation_body.content, version_content.content);

            VersionContentRepository::patch_raw(
                connection,
                article_version.content_id,
                content_delta,
            )
            .expect(FmtError::FailedToUpdate("version_content").fmt().as_str());
        }

        let version_content = VersionContentRepository::insert_raw(
            connection,
            VersionContentDto {
                content: creation_body.content.as_bytes().to_vec(),
                content_type: ContentType::Full,
            },
        )
        .expect(FmtError::FailedToInsert("version_content").fmt().as_str());

        let article_version = ArticleVersionRepository::insert_raw(
            connection,
            ArticleVersionCreateDto {
                article_language_id,
                version: article_versions_count + 1,
                content_id: version_content.id,
            },
        )
        .expect(FmtError::FailedToInsert("article_version").fmt().as_str());

        (article_version, version_content)
    }
}
