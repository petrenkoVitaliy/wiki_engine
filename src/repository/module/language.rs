use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::error::formatted_error::FmtError;

pub mod model;

pub struct LanguageRepository {}

impl LanguageRepository {
    pub async fn get_one(
        connection: &connection::PgConnection,
        code: String,
    ) -> Option<model::Language> {
        connection
            .run(|connection| {
                db_schema::language::table
                    .filter(db_schema::language::code.eq(code))
                    .first(connection)
                    .optional()
            })
            .await
            .expect(FmtError::FailedToFetch("language").fmt().as_str())
    }

    pub async fn get_many(connection: &connection::PgConnection) -> Vec<model::Language> {
        connection
            .run(|connection| db_schema::language::table.load(connection))
            .await
            .expect(FmtError::FailedToFetch("languages").fmt().as_str())
    }
}
