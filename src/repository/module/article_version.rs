use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::error::formatted_error::FmtError;
use super::wrapper;

use super::schema::article_version::ArticleVersionCreateDto;

pub mod model;

pub struct ArticleVersionRepository {}

impl ArticleVersionRepository {
    pub async fn get_many(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Vec<model::ArticleVersion> {
        connection
            .run(|connection| {
                db_schema::article_version::table
                    .filter(
                        db_schema::article_version::article_language_id
                            .eq_any(article_languages_ids),
                    )
                    .load(connection)
            })
            .await
            .expect(FmtError::FailedToFetch("article_versions").fmt().as_str())
    }

    pub async fn _insert(
        connection: &connection::PgConnection,
        creation_dto: ArticleVersionCreateDto,
    ) -> model::ArticleVersion {
        wrapper::_wrap_db(
            &connection,
            ArticleVersionRepository::insert_raw,
            creation_dto,
            FmtError::FailedToInsert("article"),
        )
        .await
    }

    pub fn insert_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleVersionCreateDto,
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
