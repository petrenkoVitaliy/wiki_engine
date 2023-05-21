use diesel::prelude::*;

use super::db_schema;

pub mod model;

pub struct ArticleRepository {}

impl ArticleRepository {
    pub fn get_one(
        connection: &mut diesel::PgConnection,
        article_id: i32,
    ) -> Result<Option<model::Article>, diesel::result::Error> {
        db_schema::article::table
            .filter(db_schema::article::id.eq(article_id))
            .first(connection)
            .optional()
    }

    pub fn get_many(
        connection: &mut diesel::PgConnection,
        _dto: (),
    ) -> Result<Vec<model::Article>, diesel::result::Error> {
        db_schema::article::table.load(connection)
    }

    pub fn insert(
        connection: &mut diesel::PgConnection,
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
