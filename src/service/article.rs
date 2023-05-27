use diesel::Connection;

use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::article_language::ArticleLanguageService;
use super::language::LanguageService;

use super::schema::article::{ArticleAggregation, ArticleCreateRelationsDto, ArticlePatchDto};
use super::schema::article_language::ArticleLanguageCreateDto;
use super::schema::article_version::ArticleVersionCreateDto;

use super::repository::connection;
use super::repository::module::{
    article::{model, ArticleRepository},
    article_language::{model::ArticleLanguage, ArticleLanguageRepository},
    article_version::{model::ArticleVersion, ArticleVersionRepository},
};

use super::mapper::article::ArticleMapper;

pub struct ArticleService {}

impl ArticleService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        id: i32,
        query_options: QueryOptions,
    ) -> Option<ArticleAggregation> {
        let article = match ArticleRepository::get_one(connection, id, &query_options).await {
            None => return None,
            Some(article) => article,
        };

        let article_language_aggregations = ArticleLanguageService::get_aggregations(
            &connection,
            vec![article.id],
            QueryOptions { is_actual: false },
        )
        .await;

        Some(ArticleMapper::map_to_full_aggregation(
            &article,
            article_language_aggregations,
        ))
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
        query_options: QueryOptions,
    ) -> Vec<ArticleAggregation> {
        let articles = ArticleRepository::get_many(connection, &query_options).await;

        let articles_ids: Vec<i32> = articles.iter().map(|article| article.id).collect();

        let mut article_language_aggregations_map =
            ArticleLanguageService::get_aggregations_map(&connection, articles_ids).await;

        articles
            .iter()
            .map(|article| {
                let article_languages_aggregations = article_language_aggregations_map
                    .remove(&article.id)
                    .unwrap_or(vec![]);

                ArticleMapper::map_to_full_aggregation(article, article_languages_aggregations)
            })
            .collect()
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        creation_dto: ArticleCreateRelationsDto,
    ) -> Option<ArticleAggregation> {
        let language_code = String::from(&creation_dto.language);

        let language = LanguageService::get_one(connection, language_code)
            .await
            .expect(FmtError::NotFound("language").fmt().as_str());

        let (article, article_language, article_version) =
            ArticleService::create_relations_transaction(connection, creation_dto, language.id)
                .await;

        let article_aggregation = ArticleMapper::map_relations_to_aggregation(
            article,
            article_language,
            article_version,
            language,
        );

        Some(article_aggregation)
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        patch_dto: ArticlePatchDto,
    ) -> Option<ArticleAggregation> {
        let article_id = patch_dto.id;

        let updated_count = ArticleRepository::patch(connection, patch_dto).await;

        if updated_count == 0 {
            return None;
        }

        ArticleService::get_aggregation(connection, article_id, QueryOptions { is_actual: false })
            .await
    }

    async fn create_relations_transaction(
        connection: &connection::PgConnection,
        creation_dto: ArticleCreateRelationsDto,
        language_id: i32,
    ) -> (model::Article, ArticleLanguage, ArticleVersion) {
        connection
            .run(move |connection| {
                return connection.transaction::<(model::Article, ArticleLanguage, ArticleVersion), diesel::result::Error, _>(
                    |transaction_connection| {
                        Ok(ArticleService::create_relations(
                            transaction_connection,
                            creation_dto,
                            language_id,
                        ))
                    },
                );
            })
            .await
            .expect("failed to create article relations")
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleCreateRelationsDto,
        language_id: i32,
    ) -> (model::Article, ArticleLanguage, ArticleVersion) {
        let article = ArticleRepository::insert_raw(connection, ())
            .expect(FmtError::FailedToInsert("article").fmt().as_str());

        let article_language = ArticleLanguageRepository::insert_raw(
            connection,
            ArticleLanguageCreateDto {
                name: creation_dto.name,
                article_id: article.id,
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

        (article, article_language, article_version)
    }
}
