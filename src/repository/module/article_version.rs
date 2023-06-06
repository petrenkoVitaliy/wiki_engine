use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::error::formatted_error::FmtError;
use super::option_config::query_options::QueryOptions;
use super::wrapper;

use super::schema::article_version::{
    ArticleVersionCreateDto, ArticleVersionPatchDto, ArticleVersionSearchDto,
    ArticleVersionsSearchDto,
};

pub mod model;

pub struct ArticleVersionRepository {}

impl ArticleVersionRepository {
    pub async fn get_count(connection: &connection::PgConnection, article_language_id: i32) -> i32 {
        let count: i64 = connection
            .run(move |connection| {
                return db_schema::article_version::table
                    .filter(db_schema::article_version::article_language_id.eq(article_language_id))
                    .count()
                    .get_result::<i64>(connection);
            })
            .await
            .expect(FmtError::FailedToFetch("article_versions").fmt().as_str());

        return count as i32;
    }

    pub async fn get_one(
        connection: &connection::PgConnection,
        query_dto: ArticleVersionSearchDto,
        query_options: &QueryOptions,
    ) -> Option<model::ArticleVersion> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let mut query = db_schema::article_version::table.into_boxed();

                if let Some(id) = query_dto.id {
                    query = query.filter(db_schema::article_version::id.eq(id));
                }

                if let Some(article_languages_ids) = query_dto.article_languages_ids {
                    query = query.filter(
                        db_schema::article_version::article_language_id
                            .eq_any(article_languages_ids),
                    );
                }

                if is_actual {
                    query = query.filter(db_schema::article_version::enabled.eq(true))
                }

                return query.first(connection).optional();
            })
            .await
            .expect(FmtError::FailedToFetch("article_version").fmt().as_str())
    }

    pub async fn get_many(
        connection: &connection::PgConnection,
        query_dto: ArticleVersionsSearchDto,
        query_options: &QueryOptions,
    ) -> Vec<model::ArticleVersion> {
        let is_actual = query_options.is_actual;

        connection
            .run(move |connection| {
                let mut query = db_schema::article_version::table.into_boxed();

                if let Some(ids) = query_dto.ids {
                    query = query.filter(db_schema::article_version::id.eq_any(ids));
                }

                if let Some(article_languages_ids) = query_dto.article_languages_ids {
                    query = query.filter(
                        db_schema::article_version::article_language_id
                            .eq_any(article_languages_ids),
                    );
                }

                if is_actual {
                    query = query.filter(db_schema::article_version::enabled.eq(true))
                }

                return query.load(connection);
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
            Self::insert_raw,
            creation_dto,
            FmtError::FailedToInsert("article_version"),
        )
        .await
    }

    pub fn get_by_version_raw(
        connection: &mut diesel::PgConnection,
        version: i32,
    ) -> Result<Option<model::ArticleVersion>, diesel::result::Error> {
        let mut query = db_schema::article_version::table.into_boxed();

        query = query.filter(db_schema::article_version::version.eq(version));

        return query.first(connection).optional();
    }

    pub fn insert_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: ArticleVersionCreateDto,
    ) -> Result<model::ArticleVersion, diesel::result::Error> {
        diesel::insert_into(db_schema::article_version::table)
            .values(model::ArticleVersionInsertable {
                id: None,

                content_id: creation_dto.content_id,
                version: creation_dto.version,
                article_language_id: creation_dto.article_language_id,
                enabled: Some(true),

                updated_at: None,
                created_at: None,
            })
            .get_result::<model::ArticleVersion>(connection)
    }

    pub async fn patch(
        connection: &connection::PgConnection,
        id: i32,
        article_language_id: i32,
        patch_dto: ArticleVersionPatchDto,
    ) -> model::ArticleVersion {
        connection
            .run(move |connection| {
                diesel::update(db_schema::article_version::table)
                    .filter(db_schema::article_version::id.eq(id).and(
                        db_schema::article_version::article_language_id.eq(article_language_id),
                    ))
                    .set(model::ArticleVersionPatch {
                        enabled: Some(patch_dto.enabled),

                        id: None,
                        content_id: None,
                        version: None,
                        article_language_id: None,
                        updated_at: None,
                        created_at: None,
                    })
                    .get_result::<model::ArticleVersion>(connection)
            })
            .await
            .expect(FmtError::FailedToUpdate("article_version").fmt().as_str())
    }
}
