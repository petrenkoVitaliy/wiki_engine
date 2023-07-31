use diesel::prelude::*;

use super::connection::PgConnection;
use super::error::FmtError;

use super::db_schema;
use super::model;

pub struct LanguageRepository;

impl LanguageRepository {
    pub async fn get_one(connection: &PgConnection, code: String) -> Option<model::Language> {
        connection
            .run(|connection| {
                db_schema::language::table
                    .filter(db_schema::language::code.eq(code))
                    .first(connection)
                    .optional()
            })
            .await
            .expect(&FmtError::FailedToFetch("language").fmt())
    }

    pub async fn get_many(connection: &PgConnection) -> Vec<model::Language> {
        connection
            .run(|connection| db_schema::language::table.load(connection))
            .await
            .expect(&FmtError::FailedToFetch("languages").fmt())
    }
}
