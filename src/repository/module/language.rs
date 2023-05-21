use diesel::prelude::*;

use super::db_schema;

pub mod model;

pub struct LanguageRepository {}

impl LanguageRepository {
    pub fn get_one(
        connection: &mut diesel::PgConnection,
        code: String,
    ) -> Result<Option<model::Language>, diesel::result::Error> {
        db_schema::language::table
            .filter(db_schema::language::code.eq(code))
            .first(connection)
            .optional()
    }

    pub fn get_many(
        connection: &mut diesel::PgConnection,
        _dto: (),
    ) -> Result<Vec<model::Language>, diesel::result::Error> {
        db_schema::language::table.load(connection)
    }
}
