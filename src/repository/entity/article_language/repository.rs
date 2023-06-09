use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::model;

use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::schema::article_language::{ArticleLanguageCreateDto, ArticleLanguagePatchDto};

pub struct ArticleLanguageRepository {}

impl ArticleLanguageRepository {
    pub async fn get_one(
        connection: &connection::PgConnection,
        article_id: i32,
        language_id: i32,
        query_options: &QueryOptions,
    ) -> Option<model::ArticleLanguage> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let filter_query = db_schema::article_language::article_id
                    .eq(article_id)
                    .and(db_schema::article_language::language_id.eq(language_id));

                if is_actual {
                    return db_schema::article_language::table
                        .filter(
                            filter_query
                                .and(db_schema::article_language::enabled.eq(true))
                                .and(db_schema::article_language::archived.eq(false)),
                        )
                        .first(connection)
                        .optional();
                }

                db_schema::article_language::table
                    .filter(filter_query)
                    .first(connection)
                    .optional()
            })
            .await
            .expect(FmtError::FailedToFetch("article_language").fmt().as_str())
    }

    pub async fn get_many(
        connection: &connection::PgConnection,
        article_ids: Vec<i32>,
        query_options: &QueryOptions,
    ) -> Vec<model::ArticleLanguage> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let mut query = db_schema::article_language::table.into_boxed();

                query = if article_ids.len() == 1 {
                    query.filter(db_schema::article_language::article_id.eq(article_ids[0]))
                } else {
                    query.filter(db_schema::article_language::article_id.eq_any(article_ids))
                };

                if is_actual {
                    query = query.filter(
                        db_schema::article_language::enabled
                            .eq(true)
                            .and(db_schema::article_language::archived.eq(false)),
                    );
                }

                return query
                    .order(db_schema::article_language::created_at.desc())
                    .load(connection);
            })
            .await
            .expect(FmtError::FailedToFetch("article_languages").fmt().as_str())
    }

    pub fn insert_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleLanguageCreateDto,
    ) -> Result<model::ArticleLanguage, diesel::result::Error> {
        diesel::insert_into(db_schema::article_language::table)
            .values(model::ArticleLanguageInsertable {
                id: None,

                name: String::from(creation_dto.name),
                article_id: creation_dto.article_id,
                language_id: creation_dto.language_id,

                enabled: Some(true),
                archived: Some(false),
                updated_at: None,
                created_at: None,
            })
            .get_result::<model::ArticleLanguage>(connection)
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        language_id: i32,
        article_id: i32,
        patch_dto: ArticleLanguagePatchDto,
    ) -> usize {
        connection
            .run(move |connection| {
                diesel::update(db_schema::article_language::table)
                    .filter(
                        db_schema::article_language::language_id
                            .eq(language_id)
                            .and(db_schema::article_language::article_id.eq(article_id)),
                    )
                    .set(model::ArticleLanguagePatch {
                        name: patch_dto.name,
                        enabled: patch_dto.enabled,
                        archived: patch_dto.archived,

                        id: None,
                        article_id: None,
                        language_id: None,
                        updated_at: None,
                        created_at: None,
                    })
                    .execute(connection)
            })
            .await
            .expect(FmtError::FailedToUpdate("article_language").fmt().as_str())
    }
}
