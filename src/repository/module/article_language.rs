use diesel::prelude::*;

use super::connection::PgConnection;
use super::db_schema;
use super::error::formatted_error::FmtError;
use super::schema::article_language::CreateArticleLanguageDto;
use super::wrapper;

pub mod model;

pub struct ArticleLanguageRepository {}

impl ArticleLanguageRepository {
    // TODO test eq & eq_any perf
    pub async fn get_many(
        connection: &PgConnection,
        article_ids: Vec<i32>,
    ) -> Vec<model::ArticleLanguage> {
        connection
            .run(|connection| {
                db_schema::article_language::table
                    .filter(db_schema::article_language::article_id.eq_any(article_ids))
                    .load(connection)
            })
            .await
            .expect(FmtError::FailedToFetch("article_languages").fmt().as_str())
    }

    pub async fn _insert(
        connection: &PgConnection,
        creation_dto: CreateArticleLanguageDto,
    ) -> model::ArticleLanguage {
        wrapper::_wrap_db(
            &connection,
            ArticleLanguageRepository::insert_raw,
            creation_dto,
            FmtError::FailedToInsert("article_language"),
        )
        .await
    }

    pub fn insert_raw(
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
