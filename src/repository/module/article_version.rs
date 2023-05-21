use diesel::prelude::*;

use super::db_schema;
use super::schema::article_version::CreateArticleVersionDto;

pub mod model;

pub struct ArticleVersionRepository {}

impl ArticleVersionRepository {
    pub fn get_many_by_languages(
        connection: &mut diesel::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Result<Vec<model::ArticleVersion>, diesel::result::Error> {
        db_schema::article_version::table
            .filter(db_schema::article_version::article_language_id.eq_any(article_languages_ids))
            .load(connection)
    }

    pub fn insert(
        connection: &mut diesel::PgConnection,
        creation_dto: CreateArticleVersionDto,
    ) -> Result<model::ArticleVersion, diesel::result::Error> {
        diesel::insert_into(db_schema::article_version::table)
            .values(model::ArticleVersionInsertable {
                id: None,

                content: String::from(creation_dto.content),
                version: creation_dto.version,
                article_language_id: creation_dto.article_language_id,
                enabled: Some(true),

                updated_at: None,
                created_at: None,
            })
            .get_result::<model::ArticleVersion>(connection)
    }
}
