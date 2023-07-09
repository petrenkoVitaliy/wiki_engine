use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::decorator::connection_result;
use super::model;

use super::error::formatted_error::FmtError;

use super::schema::article_version::{
    ArticleVersionCreateDto, ArticleVersionPatchDto, ArticleVersionsJoinSearchDto,
};

use super::version_content::VersionContent;

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

    pub async fn get_many_with_content(
        connection: &connection::PgConnection,
        query_dto: ArticleVersionsJoinSearchDto,
    ) -> Vec<(model::ArticleVersion, VersionContent)> {
        connection
            .run(move |connection| {
                let mut query = db_schema::article_version::table
                    .inner_join(db_schema::version_content::table)
                    .into_boxed();

                if let Some(version_ge) = query_dto.version_ge {
                    query = query.filter(db_schema::article_version::version.ge(version_ge));
                }

                if let Some(article_languages_ids) = query_dto.article_languages_ids {
                    query = query.filter(
                        db_schema::article_version::article_language_id
                            .eq_any(article_languages_ids),
                    );
                }

                return query
                    .order(db_schema::article_version::version.asc())
                    .load::<(model::ArticleVersion, VersionContent)>(connection);
            })
            .await
            .expect(FmtError::FailedToFetch("article_versions").fmt().as_str())
    }

    pub async fn _insert(
        connection: &connection::PgConnection,
        creation_dto: ArticleVersionCreateDto,
    ) -> model::ArticleVersion {
        connection_result::_wrap_db(
            &connection,
            Self::insert_raw,
            creation_dto,
            FmtError::FailedToInsert("article_version"),
        )
        .await
    }

    pub fn get_by_version_raw(
        connection: &mut diesel::PgConnection,
        article_language_id: i32,
        version: i32,
    ) -> Result<Option<model::ArticleVersion>, diesel::result::Error> {
        let mut query = db_schema::article_version::table.into_boxed();

        query = query.filter(
            db_schema::article_version::version
                .eq(version)
                .and(db_schema::article_version::article_language_id.eq(article_language_id)),
        );

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
        version: i32,
        article_language_id: i32,
        patch_dto: ArticleVersionPatchDto,
    ) -> usize {
        connection
            .run(move |connection| {
                diesel::update(db_schema::article_version::table)
                    .filter(db_schema::article_version::version.eq(version).and(
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
                    .execute(connection)
            })
            .await
            .expect(FmtError::FailedToUpdate("article_version").fmt().as_str())
    }
}
