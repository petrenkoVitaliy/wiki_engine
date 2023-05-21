use diesel::Connection;

use rocket::serde::json::Json;

use super::article_language::ArticleLanguageService;
use super::article_version::ArticleVersionService;
use super::language::LanguageService;

use super::schema::article::CreateArticleDto;
use super::schema::article_language::{ArticleLanguageAggregation, CreateArticleLanguageDto};
use super::schema::article_version::CreateArticleVersionDto;
use crate::schema::article::ArticleAggregation;

use super::repository::module::language::model::Language;
use super::repository::module::{
    article::{model, ArticleRepository},
    article_language::{model::ArticleLanguage, ArticleLanguageRepository},
    article_version::{model::ArticleVersion, ArticleVersionRepository},
    language::LanguageRepository,
};

use super::repository::connection;

pub struct ArticleService {}

// TODO add errors generator

impl ArticleService {
    // TODO r u drunk?
    async fn get_one(connection: &connection::PgConnection, article_id: i32) -> model::Article {
        connection::wrap_db(
            &connection,
            ArticleRepository::get_one,
            article_id,
            "failed to fetch article",
        )
        .await
        .expect("article wasn't found")
    }

    async fn get_many(connection: &connection::PgConnection) -> Vec<model::Article> {
        connection::wrap_db(
            &connection,
            ArticleRepository::get_many,
            (),
            "failed to fetch articles",
        )
        .await
    }

    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        article_id: i32,
    ) -> ArticleAggregation {
        let article = ArticleService::get_one(connection, article_id).await;

        let article_language_aggregations =
            ArticleLanguageService::get_aggregations(&connection, vec![article.id]).await;

        ArticleService::map_to_full_aggregation(&article, article_language_aggregations)
    }

    pub async fn get_aggregations(
        connection: &connection::PgConnection,
    ) -> Vec<ArticleAggregation> {
        let articles = ArticleService::get_many(connection).await;

        let articles_ids: Vec<i32> = articles.iter().map(|article| article.id).collect();

        let mut article_language_aggregations_map =
            ArticleLanguageService::get_aggregations_map(&connection, articles_ids).await;

        articles
            .iter()
            .map(|article| {
                let article_languages_aggregations = article_language_aggregations_map
                    .remove(&article.id)
                    .unwrap_or(vec![]);

                ArticleService::map_to_full_aggregation(article, article_languages_aggregations)
            })
            .collect()
    }

    pub async fn insert(
        connection: &connection::PgConnection,
        article_dto: Json<CreateArticleDto>,
    ) -> Option<ArticleAggregation> {
        let language_code = article_dto.language.to_string();

        let language = match connection::wrap_db(
            &connection,
            LanguageRepository::get_one,
            language_code,
            "failed to fetch language",
        )
        .await
        {
            Some(language) => language,
            _ => return None,
        };

        let (article, article_language, article_version) =
            ArticleService::create_article_relations_transaction(
                connection,
                article_dto,
                language.id,
            )
            .await;

        let article_aggregation = ArticleService::map_relations_to_aggregation(
            article,
            article_language,
            article_version,
            language,
        );

        Some(article_aggregation)
    }

    fn map_to_full_aggregation(
        article: &model::Article,
        article_language_aggregations: Vec<ArticleLanguageAggregation>,
    ) -> ArticleAggregation {
        ArticleAggregation {
            id: article.id,
            enabled: article.enabled,
            archived: article.archived,
            updated_at: article.updated_at,
            created_at: article.created_at,

            languages: article_language_aggregations,
        }
    }

    async fn create_article_relations_transaction(
        connection: &connection::PgConnection,
        article_dto: Json<CreateArticleDto>,
        language_id: i32,
    ) -> (model::Article, ArticleLanguage, ArticleVersion) {
        connection
            .run(move |connection| {
                return connection.transaction::<(model::Article, ArticleLanguage, ArticleVersion), diesel::result::Error, _>(
                    |transaction_connection| {
                        Ok(ArticleService::create_article_relations(
                            transaction_connection,
                            article_dto,
                            language_id,
                        ))
                    },
                );
            })
            .await
            .expect("failed to create article relations")
    }

    fn create_article_relations(
        connection: &mut diesel::PgConnection,
        article_dto: Json<CreateArticleDto>,
        language_id: i32,
    ) -> (model::Article, ArticleLanguage, ArticleVersion) {
        let article = ArticleRepository::insert(connection).expect("failed to create article");

        let article_language = ArticleLanguageRepository::insert(
            connection,
            CreateArticleLanguageDto {
                name: article_dto.name.to_string(),
                article_id: article.id,
                language_id: language_id,
            },
        )
        .expect("failed to create article_language");

        let article_version = ArticleVersionRepository::insert(
            connection,
            CreateArticleVersionDto {
                version: article.id,
                article_language_id: article_language.id,
                content: article_dto.content.to_string(),
            },
        )
        .expect("failed to create article_version");

        (article, article_language, article_version)
    }

    fn map_relations_to_aggregation(
        article: model::Article,
        article_language: ArticleLanguage,
        article_version: ArticleVersion,
        language: Language,
    ) -> ArticleAggregation {
        let article_versions_aggregations =
            ArticleVersionService::map_to_aggregations(vec![article_version]);

        let languages_aggregation = LanguageService::map_to_aggregations(vec![language]);
        let article_language_aggregations = ArticleLanguageService::map_to_aggregations(
            vec![article_language],
            article_versions_aggregations,
            languages_aggregation,
        );

        ArticleService::map_to_full_aggregation(&article, article_language_aggregations)
    }
}
