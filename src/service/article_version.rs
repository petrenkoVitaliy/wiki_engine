use diesel::Connection;
use std::collections::HashMap;

use super::repository::connection;
use super::repository::entity::article_version::{ArticleVersion, ArticleVersionRepository};
use super::repository::entity::version_content::{
    ContentType, VersionContent, VersionContentRepository,
};

use super::diff_handler::diff_handler::DiffHandler;
use super::error::error_wrapper::ErrorWrapper;
use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::article_language::ArticleLanguageService;
use super::version_content::VersionContentService;

use super::schema::article_version::{
    ArticleVersionCreateBody, ArticleVersionCreateDto, ArticleVersionPatchBody,
    ArticleVersionPatchDto, ArticleVersionsJoinSearchDto, LanguageSearchDto,
};
use super::schema::version_content::VersionContentDto;

use super::aggregation::article_version::ArticleVersionAggregation;

pub struct ArticleVersionService {}

impl ArticleVersionService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        version: i32,
        language_search_dto: LanguageSearchDto,
        query_options: &QueryOptions,
    ) -> Result<ArticleVersionAggregation, ErrorWrapper> {
        let (article_versions_contents, content_map) = match Self::get_versions_with_content_map(
            connection,
            Some(version),
            language_search_dto,
        )
        .await
        {
            Err(e) => return Err(e),
            Ok(versions_with_content_map) => versions_with_content_map,
        };

        let requested_article_version_with_content = match article_versions_contents
            .into_iter()
            .find(|(article_version, _)| {
                if !query_options.is_actual {
                    return article_version.version == version;
                }

                return article_version.version == version && article_version.enabled;
            }) {
            Some(article_version_with_content) => article_version_with_content,
            None => return FmtError::NotFound("article_version").error(),
        };

        let mut article_versions_aggregations = ArticleVersionAggregation::from_content_map(
            vec![requested_article_version_with_content],
            content_map,
        );

        return Ok(article_versions_aggregations.remove(0));
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        language_search_dto: LanguageSearchDto,
        query_options: &QueryOptions,
    ) -> Result<Vec<ArticleVersionAggregation>, ErrorWrapper> {
        let (article_versions_contents, content_map) = match Self::get_versions_with_content_map(
            connection,
            None,
            language_search_dto,
        )
        .await
        {
            Err(e) => return Err(e),
            Ok(versions_with_content_map) => versions_with_content_map,
        };

        let article_versions_aggregations =
            ArticleVersionAggregation::from_content_map(article_versions_contents, content_map);

        Ok(article_versions_aggregations
            .into_iter()
            .filter(move |aggregation| {
                if !query_options.is_actual {
                    return true;
                }

                return aggregation.enabled;
            })
            .collect::<Vec<ArticleVersionAggregation>>())
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        version: i32,
        article_id: i32,
        language_code: String,
        patch_body: ArticleVersionPatchBody,
    ) -> Result<ArticleVersionAggregation, ErrorWrapper> {
        let article_language = match ArticleLanguageService::get_one_with_language(
            connection,
            article_id,
            language_code,
            &QueryOptions { is_actual: true },
        )
        .await
        {
            Err(e) => return Err(e),
            Ok((article_language, _)) => article_language,
        };

        let updated_count = ArticleVersionRepository::patch(
            connection,
            version,
            article_language.id,
            ArticleVersionPatchDto {
                enabled: patch_body.enabled,
            },
        )
        .await;

        if updated_count == 0 {
            return FmtError::NotFound("article_version").error();
        }

        return Self::get_aggregation(
            connection,
            version,
            LanguageSearchDto {
                article_language: Some(article_language),

                language_code: None,
                article_languages_ids: None,
                article_id: None,
            },
            &QueryOptions { is_actual: false },
        )
        .await;
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        article_id: i32,
        language_code: String,
        creation_body: ArticleVersionCreateBody,
    ) -> Result<ArticleVersionAggregation, ErrorWrapper> {
        let article_language = match ArticleLanguageService::get_one_with_language(
            connection,
            article_id,
            language_code,
            &QueryOptions { is_actual: true },
        )
        .await
        {
            Err(e) => return Err(e),
            Ok((article_language, _)) => article_language,
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

        Ok(ArticleVersionAggregation::from_related_models(
            vec![article_version],
            vec![version_content],
        )
        .remove(0))
    }

    async fn create_relations_transaction(
        connection: &connection::PgConnection,
        creation_body: ArticleVersionCreateBody,
        article_language_id: i32,
        article_versions_count: i32,
    ) -> (ArticleVersion, VersionContent) {
        connection
            .run(move |connection| {
                return connection
                    .transaction::<(ArticleVersion, VersionContent), diesel::result::Error, _>(
                        |transaction_connection| {
                            Ok(Self::create_relations(
                                transaction_connection,
                                creation_body,
                                article_language_id,
                                article_versions_count,
                            ))
                        },
                    );
            })
            .await
            .expect(
                FmtError::FailedToInsert("article_version_relations")
                    .fmt()
                    .as_str(),
            )
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_body: ArticleVersionCreateBody,
        article_language_id: i32,
        article_versions_count: i32,
    ) -> (ArticleVersion, VersionContent) {
        if article_versions_count > 0 {
            Self::update_previous_version_content(
                connection,
                article_language_id,
                article_versions_count,
                &creation_body,
            );
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

    fn update_previous_version_content(
        connection: &mut diesel::PgConnection,
        article_language_id: i32,
        article_versions_count: i32,
        creation_body: &ArticleVersionCreateBody,
    ) {
        let article_version = ArticleVersionRepository::get_by_version_raw(
            connection,
            article_language_id,
            article_versions_count,
        )
        .expect(FmtError::FailedToFetch("article_version").fmt().as_str())
        .expect(FmtError::NotFound("article_version").fmt().as_str());

        let version_content =
            VersionContentRepository::get_one_raw(connection, article_version.content_id)
                .expect(FmtError::FailedToFetch("version_content").fmt().as_str())
                .expect(FmtError::NotFound("version_content").fmt().as_str());

        let content_delta = DiffHandler::get_delta(&creation_body.content, version_content.content);

        VersionContentRepository::patch_raw(connection, article_version.content_id, content_delta)
            .expect(FmtError::FailedToUpdate("version_content").fmt().as_str());
    }

    async fn get_versions_with_content_map(
        connection: &connection::PgConnection,
        version: Option<i32>,
        language_search_dto: LanguageSearchDto,
    ) -> Result<
        (
            Vec<(ArticleVersion, VersionContent)>,
            HashMap<i32, std::string::String>,
        ),
        ErrorWrapper,
    > {
        let article_languages_ids = match language_search_dto.article_languages_ids {
            Some(article_languages_ids) => article_languages_ids,
            None => match language_search_dto.article_language {
                Some(article_language) => vec![article_language.id],
                None => {
                    let (language_code, article_id) = match (
                        language_search_dto.language_code,
                        language_search_dto.article_id,
                    ) {
                        (Some(language_code), Some(article_id)) => (language_code, article_id),
                        _ => panic!(
                            "{}",
                            FmtError::FailedToProcess("language_code").fmt().as_str()
                        ),
                    };

                    match ArticleLanguageService::get_one_with_language(
                        connection,
                        article_id,
                        language_code,
                        &QueryOptions { is_actual: true },
                    )
                    .await
                    {
                        Err(e) => return Err(e),
                        Ok((article_language, _)) => vec![article_language.id],
                    }
                }
            },
        };

        let article_versions_contents = ArticleVersionRepository::get_many_with_content(
            connection,
            ArticleVersionsJoinSearchDto {
                version_ge: version,
                article_languages_ids: Some(article_languages_ids),
            },
        )
        .await;

        let content_map =
            VersionContentService::get_contents_map_by_ids(&article_versions_contents);

        Ok((article_versions_contents, content_map))
    }
}
