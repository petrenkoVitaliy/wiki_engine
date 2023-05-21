use diesel::prelude::*;

use super::db_schema;
use super::schema::article_language::CreateArticleLanguageDto;

pub mod model;

pub struct ArticleLanguageRepository {}

impl ArticleLanguageRepository {
    // TODO test eq & eq_any perf
    pub fn get_many_by_article(
        connection: &mut diesel::PgConnection,
        article_ids: Vec<i32>,
    ) -> Result<Vec<model::ArticleLanguage>, diesel::result::Error> {
        db_schema::article_language::table
            .filter(db_schema::article_language::article_id.eq_any(article_ids))
            .load(connection)
    }

    pub fn insert(
        connection: &mut diesel::PgConnection,
        creation_dto: CreateArticleLanguageDto,
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
}
