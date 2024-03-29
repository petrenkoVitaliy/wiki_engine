use diesel::Connection;
use std::collections::HashMap;

use super::authorization::PermissionsHandler;
use super::diff_handler::DiffHandler;
use super::dtm_common::QueryOptions;
use super::error::{ErrorWrapper, FmtError};

use super::dtm::{
    article_language::dto::ArticleLanguagePatchDto,
    article_version::dto::{
        ArticleVersionCreateDto, ArticleVersionCreateRelationsDto, ArticleVersionPatchDto,
        ArticleVersionsJoinSearchDto, LanguageSearchDto,
    },
    version_content::dto::VersionContentDto,
};

use super::aggregation::{
    article_version::ArticleVersionAggregation, user_account::UserAccountAggregation,
};

use super::repository::{
    entity::{
        article::ArticleRepository,
        article_language::{ArticleLanguage, ArticleLanguageRepository},
        article_version::{ArticleVersion, ArticleVersionRepository},
        auth::UserAccount,
        version_content::{ContentType, VersionContent, VersionContentRepository},
    },
    PgConnection,
};

use super::article_language::ArticleLanguageService;
use super::version_content::VersionContentService;

pub struct ArticleVersionService;

impl ArticleVersionService {
    pub async fn get_aggregation(
        connection: &PgConnection,
        version: Option<i32>,
        language_search_dto: LanguageSearchDto,
        query_options: &QueryOptions,
    ) -> Result<ArticleVersionAggregation, ErrorWrapper> {
        let (article_versions_relations, content_map) =
            match Self::get_versions_with_content_map(connection, version, language_search_dto)
                .await
            {
                Err(e) => return Err(e),
                Ok(versions_with_content_map) => versions_with_content_map,
            };

        let requested_article_version_relations =
            match Self::get_requested_article_version_with_content(
                article_versions_relations,
                version,
                query_options,
            ) {
                Some(article_versions_relations) => article_versions_relations,
                None => return FmtError::NotFound("article_version").error(),
            };

        let mut article_versions_aggregations = ArticleVersionAggregation::from_content_map(
            vec![requested_article_version_relations],
            content_map,
        );

        return Ok(article_versions_aggregations.swap_remove(0));
    }

    pub async fn get_aggregations(
        connection: &PgConnection,
        actual_only: bool,
        language_search_dto: LanguageSearchDto,
        query_options: &QueryOptions,
    ) -> Result<Vec<ArticleVersionAggregation>, ErrorWrapper> {
        let version_ge_to_search = if actual_only { None } else { Some(1) };
        let (article_versions_relations, content_map) = match Self::get_versions_with_content_map(
            connection,
            version_ge_to_search,
            language_search_dto,
        )
        .await
        {
            Err(e) => return Err(e),
            Ok(versions_with_content_map) => versions_with_content_map,
        };

        let article_versions_aggregations =
            ArticleVersionAggregation::from_content_map(article_versions_relations, content_map);

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
        connection: &PgConnection,
        version: i32,
        article_id: i32,
        language_code: String,
        patch_dto: ArticleVersionPatchDto,
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

        let updated_count =
            ArticleVersionRepository::patch(connection, version, article_language.id, patch_dto)
                .await;

        if updated_count == 0 {
            return FmtError::NotFound("article_version").error();
        }

        return Self::get_aggregation(
            connection,
            Some(version),
            LanguageSearchDto {
                article_language: Some(article_language),

                language_code: None,
                article_languages_ids: None,
                article_id: None,
                article_language_key: None,
            },
            &QueryOptions { is_actual: false },
        )
        .await;
    }

    pub async fn insert(
        connection: &PgConnection,
        article_id: i32,
        language_code: String,
        creation_dto: ArticleVersionCreateRelationsDto,
        user_aggregation: &UserAccountAggregation,
    ) -> Result<ArticleVersionAggregation, ErrorWrapper> {
        let article = match ArticleRepository::get_one(
            connection,
            article_id,
            &QueryOptions { is_actual: false },
        )
        .await
        {
            Some(article) => article,
            None => return FmtError::NotFound("article").error(),
        };

        match PermissionsHandler::can_create_article_version(&article, user_aggregation) {
            false => return FmtError::PermissionDenied("not enough rights").error(),
            _ => (),
        };

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
            creation_dto,
            article_language,
            article_versions_count,
        )
        .await;

        Ok(ArticleVersionAggregation::from_related_models(
            vec![article_version],
            vec![version_content],
        )
        .swap_remove(0))
    }

    async fn create_relations_transaction(
        connection: &PgConnection,
        creation_dto: ArticleVersionCreateRelationsDto,
        article_language: ArticleLanguage,
        article_versions_count: i32,
    ) -> (ArticleVersion, VersionContent) {
        connection
            .run(move |connection| {
                return connection
                    .transaction::<(ArticleVersion, VersionContent), diesel::result::Error, _>(
                        |transaction_connection| {
                            Ok(Self::create_relations(
                                transaction_connection,
                                creation_dto,
                                article_language,
                                article_versions_count,
                            ))
                        },
                    );
            })
            .await
            .expect(&FmtError::FailedToInsert("article_version_relations").fmt())
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleVersionCreateRelationsDto,
        article_language: ArticleLanguage,
        article_versions_count: i32,
    ) -> (ArticleVersion, VersionContent) {
        if article_versions_count > 0 {
            Self::update_previous_version_content(
                connection,
                article_language.id,
                article_versions_count,
                &creation_dto,
            );
        }

        let actual_language_name = match creation_dto.name {
            Some(name) => {
                if name != article_language.name {
                    ArticleLanguageRepository::patch_raw(
                        connection,
                        article_language.id,
                        ArticleLanguagePatchDto {
                            name: Some(name.clone()),
                            user_id: creation_dto.user_id,
                            enabled: None,
                            archived: None,
                        },
                    )
                    .expect(&FmtError::FailedToUpdate("article_language").fmt());
                }

                name
            }
            _ => article_language.name,
        };

        let version_content = VersionContentRepository::insert_raw(
            connection,
            VersionContentDto {
                content: creation_dto.content.as_bytes().to_vec(),
                content_type: ContentType::Full,
            },
        )
        .expect(&FmtError::FailedToInsert("version_content").fmt());

        let article_version = ArticleVersionRepository::insert_raw(
            connection,
            ArticleVersionCreateDto {
                article_language_id: article_language.id,
                version: article_versions_count + 1,
                content_id: version_content.id,
                user_id: creation_dto.user_id,
                name: actual_language_name,
            },
        )
        .expect(&FmtError::FailedToInsert("article_version").fmt());

        (article_version, version_content)
    }

    fn update_previous_version_content(
        connection: &mut diesel::PgConnection,
        article_language_id: i32,
        article_versions_count: i32,
        creation_dto: &ArticleVersionCreateRelationsDto,
    ) {
        let article_version = ArticleVersionRepository::get_by_version_raw(
            connection,
            article_language_id,
            article_versions_count,
        )
        .expect(&FmtError::FailedToFetch("article_version").fmt())
        .expect(&FmtError::NotFound("article_version").fmt());

        let version_content =
            VersionContentRepository::get_one_raw(connection, article_version.content_id)
                .expect(&FmtError::FailedToFetch("version_content").fmt())
                .expect(&FmtError::NotFound("version_content").fmt());

        let content_delta = DiffHandler::get_delta(&creation_dto.content, version_content.content);

        VersionContentRepository::patch_raw(connection, article_version.content_id, content_delta)
            .expect(&FmtError::FailedToUpdate("version_content").fmt());
    }

    fn get_requested_article_version_with_content(
        article_versions_relations: Vec<(ArticleVersion, VersionContent, UserAccount)>,
        version: Option<i32>,
        query_options: &QueryOptions,
    ) -> Option<(ArticleVersion, VersionContent, UserAccount)> {
        article_versions_relations
            .into_iter()
            .find(|(article_version, _, _)| {
                if let Some(version) = version {
                    if query_options.is_actual {
                        return article_version.version == version && article_version.enabled;
                    }

                    return article_version.version == version;
                }

                return query_options.is_actual && article_version.enabled
                    || !query_options.is_actual;
            })
    }

    async fn get_versions_with_content_map(
        connection: &PgConnection,
        version: Option<i32>,
        language_search_dto: LanguageSearchDto,
    ) -> Result<
        (
            Vec<(ArticleVersion, VersionContent, UserAccount)>,
            HashMap<i32, std::string::String>,
        ),
        ErrorWrapper,
    > {
        let article_languages_ids = match language_search_dto.article_languages_ids {
            Some(article_languages_ids) => article_languages_ids,
            None => match language_search_dto.article_language {
                Some(article_language) => vec![article_language.id],
                None => match language_search_dto.article_language_key {
                    Some(article_language_key) => {
                        match ArticleLanguageService::get_one_by_key(
                            connection,
                            article_language_key,
                            &QueryOptions { is_actual: true },
                        )
                        .await
                        {
                            Err(e) => return Err(e),
                            Ok(article_language) => vec![article_language.id],
                        }
                    }
                    None => {
                        let (language_code, article_id) = match (
                            language_search_dto.language_code,
                            language_search_dto.article_id,
                        ) {
                            (Some(language_code), Some(article_id)) => (language_code, article_id),
                            _ => panic!("{}", &FmtError::FailedToProcess("language_code").fmt()),
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
            },
        };

        let article_versions_relations = match version {
            Some(version) => {
                ArticleVersionRepository::get_many_with_content(
                    connection,
                    ArticleVersionsJoinSearchDto {
                        article_languages_ids,
                        version_ge: version,
                    },
                )
                .await
            }
            None => {
                ArticleVersionRepository::get_many_actuals_with_content(
                    connection,
                    article_languages_ids,
                )
                .await
            }
        };

        let content_map =
            VersionContentService::get_contents_map_by_ids(&article_versions_relations);

        Ok((article_versions_relations, content_map))
    }
}
