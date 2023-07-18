use diesel::prelude::*;

use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;

use super::connection;
use super::db_schema;
use super::model;

use super::schema::article::ArticlePatchDto;

pub struct ArticleRepository {}

impl ArticleRepository {
    pub async fn get_one(
        connection: &connection::PgConnection,
        id: i32,
        query_options: &QueryOptions,
    ) -> Option<model::Article> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let query = db_schema::article::table.filter(db_schema::article::id.eq(id));

                if is_actual {
                    return query
                        .filter(db_schema::article::enabled.eq(true))
                        .filter(db_schema::article::archived.eq(false))
                        .first(connection)
                        .optional();
                }

                query.first(connection).optional()
            })
            .await
            .expect(&FmtError::FailedToFetch("article").fmt())
    }

    pub async fn get_many(
        connection: &connection::PgConnection,
        query_options: &QueryOptions,
    ) -> Vec<model::Article> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let query = db_schema::article::table;

                if is_actual {
                    return query
                        .filter(db_schema::article::enabled.eq(true))
                        .filter(db_schema::article::archived.eq(false))
                        .load(connection);
                }

                query
                    .order(db_schema::article::created_at.desc())
                    .load(connection)
            })
            .await
            .expect(&FmtError::FailedToFetch("articles").fmt())
    }

    pub async fn patch(connection: &connection::PgConnection, patch_dto: ArticlePatchDto) -> usize {
        connection
            .run(move |connection| {
                diesel::update(db_schema::article::table)
                    .filter(db_schema::article::id.eq(patch_dto.id))
                    .set(model::ArticleInsertable {
                        id: None,
                        enabled: patch_dto.enabled,
                        archived: patch_dto.archived,
                        updated_at: None,
                        created_at: None,
                    })
                    .execute(connection)
            })
            .await
            .expect(&FmtError::FailedToUpdate("article").fmt())
    }

    pub fn insert_raw(
        connection: &mut diesel::PgConnection,
        _dto: (),
    ) -> Result<model::Article, diesel::result::Error> {
        diesel::insert_into(db_schema::article::table)
            .values(model::ArticleInsertable {
                id: None,
                enabled: Some(true),
                archived: Some(false),
                updated_at: None,
                created_at: None,
            })
            .get_result::<model::Article>(connection)
    }
}
