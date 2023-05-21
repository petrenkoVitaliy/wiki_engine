use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::error::formatted_error::FmtError;
use super::wrapper;

pub mod model;

pub struct ArticleRepository {}

impl ArticleRepository {
    pub async fn get_one(
        connection: &connection::PgConnection,
        article_id: i32,
    ) -> Option<model::Article> {
        connection
            .run(move |connection| {
                db_schema::article::table
                    .filter(db_schema::article::id.eq(article_id))
                    .first(connection)
                    .optional()
            })
            .await
            .expect(FmtError::FailedToFetch("article").fmt().as_str())
    }

    pub async fn get_many(connection: &connection::PgConnection) -> Vec<model::Article> {
        connection
            .run(|connection| db_schema::article::table.load(connection))
            .await
            .expect(FmtError::FailedToFetch("articles").fmt().as_str())
    }

    pub async fn _insert(connection: &connection::PgConnection) -> model::Article {
        wrapper::wrap_db(
            &connection,
            ArticleRepository::insert_raw,
            (),
            FmtError::FailedToInsert("article"),
        )
        .await
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
