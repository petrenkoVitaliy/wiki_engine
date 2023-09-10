use diesel::Connection;

use super::dtm_common::QueryOptions;
use super::error::{ErrorWrapper, FmtError};

use super::dtm::{
    article::dto::{ArticleCreateDto, ArticleCreateRelationsDto, ArticlePatchDto},
    article_language::dto::ArticleLanguageCreateDto,
    article_version::dto::ArticleVersionCreateDto,
    version_content::dto::VersionContentDto,
};

use super::aggregation::{article::ArticleAggregation, user_account::UserAccountAggregation};
use super::authorization::PermissionsHandler;

use super::repository::{
    entity::{
        article::{Article, ArticleRepository},
        article_language::{ArticleLanguage, ArticleLanguageRepository},
        article_version::{ArticleVersion, ArticleVersionRepository},
        version_content::{ContentType, VersionContent, VersionContentRepository},
    },
    PgConnection,
};

use super::article_language::ArticleLanguageService;
use super::language::LanguageService;

pub struct ArticleService;

impl ArticleService {
    pub async fn get_aggregation(
        connection: &PgConnection,
        id: i32,
        query_options: &QueryOptions,
    ) -> Result<ArticleAggregation, ErrorWrapper> {
        let article = match ArticleRepository::get_one(connection, id, query_options).await {
            None => return FmtError::NotFound("article").error(),
            Some(article) => article,
        };

        let article_language_aggregations =
            ArticleLanguageService::get_aggregations(&connection, article.id, query_options).await;

        Ok(ArticleAggregation::from_model(
            article,
            article_language_aggregations,
        ))
    }

    pub async fn get_aggregation_by_key(
        connection: &PgConnection,
        article_language_key: String,
        query_options: &QueryOptions,
    ) -> Result<ArticleAggregation, ErrorWrapper> {
        let (_, article) = match ArticleRepository::get_one_by_key(
            connection,
            article_language_key,
            query_options,
        )
        .await
        {
            None => return FmtError::NotFound("article").error(),
            Some(article) => article,
        };

        let article_language_aggregations =
            ArticleLanguageService::get_aggregations(&connection, article.id, query_options).await;

        Ok(ArticleAggregation::from_model(
            article,
            article_language_aggregations,
        ))
    }

    pub async fn get_aggregations(
        connection: &PgConnection,
        query_options: &QueryOptions,
    ) -> Vec<ArticleAggregation> {
        let articles = ArticleRepository::get_many(connection, query_options).await;

        let articles_ids = articles.iter().map(|article| article.id).collect();

        let article_language_aggregations_map =
            ArticleLanguageService::get_aggregations_map(&connection, articles_ids, query_options)
                .await;

        ArticleAggregation::from_languages_map(articles, article_language_aggregations_map)
    }

    pub async fn insert(
        connection: &PgConnection,
        creation_dto: ArticleCreateRelationsDto,
    ) -> Result<ArticleAggregation, ErrorWrapper> {
        let language_code = String::from(&creation_dto.language);

        let language = match LanguageService::get_one(connection, language_code).await {
            None => return FmtError::NotFound("language").error(),
            Some(language) => language,
        };

        let (article, article_language, version_content, article_version) =
            Self::create_relations_transaction(connection, creation_dto, language.id).await;

        let article_aggregation = ArticleAggregation::from_related_models(
            article,
            article_language,
            article_version,
            version_content,
            language,
        );

        Ok(article_aggregation)
    }

    pub async fn patch(
        connection: &PgConnection,
        patch_dto: ArticlePatchDto,
        user_aggregation: &UserAccountAggregation,
    ) -> Result<ArticleAggregation, ErrorWrapper> {
        let article_id = patch_dto.id;

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

        match PermissionsHandler::can_patch_article(&article, user_aggregation) {
            false => return FmtError::PermissionDenied("not enough rights").error(),
            _ => (),
        };

        let updated_count = ArticleRepository::patch(connection, patch_dto).await;

        if updated_count == 0 {
            return FmtError::NotFound("article").error();
        }

        Self::get_aggregation(connection, article_id, &QueryOptions { is_actual: false }).await
    }

    async fn create_relations_transaction(
        connection: &PgConnection,
        creation_dto: ArticleCreateRelationsDto,
        language_id: i32,
    ) -> (Article, ArticleLanguage, VersionContent, ArticleVersion) {
        connection
            .run(move |connection| {
                return connection.transaction::<(
                    Article,
                    ArticleLanguage,
                    VersionContent,
                    ArticleVersion,
                ), diesel::result::Error, _>(
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
            .expect(&FmtError::FailedToInsert("article_relations").fmt())
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleCreateRelationsDto,
        language_id: i32,
    ) -> (Article, ArticleLanguage, VersionContent, ArticleVersion) {
        let article = ArticleRepository::insert_raw(
            connection,
            ArticleCreateDto {
                article_type: creation_dto.article_type,
                user_id: creation_dto.user_id,
            },
        )
        .expect(&FmtError::FailedToInsert("article").fmt());

        let article_language = ArticleLanguageRepository::insert_raw(
            connection,
            ArticleLanguageCreateDto {
                name: creation_dto.name.clone(),
                article_id: article.id,
                language_id: language_id,
                user_id: creation_dto.user_id,
            },
        )
        .expect(&FmtError::FailedToInsert("article_language").fmt());

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
                version: 1,
                article_language_id: article_language.id,
                content_id: version_content.id,
                user_id: creation_dto.user_id,
                name: creation_dto.name,
            },
        )
        .expect(&FmtError::FailedToInsert("article_version").fmt());

        (article, article_language, version_content, article_version)
    }
}
