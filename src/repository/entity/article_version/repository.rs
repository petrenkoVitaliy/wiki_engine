use diesel::{prelude::*, sql_query};

use super::connection;
use super::db_schema;
use super::model;

use super::error::formatted_error::FmtError;

use super::schema::article_version::{
    ArticleVersionCreateDto, ArticleVersionPatchDto, ArticleVersionsJoinSearchDto,
};

use super::version_content::VersionContent;

pub struct ArticleVersionRepository;

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
            .expect(&FmtError::FailedToFetch("article_versions").fmt());

        return count as i32;
    }

    pub async fn get_many_actuals_with_content(
        connection: &connection::PgConnection,
        article_languages_ids: Vec<i32>,
    ) -> Vec<(model::ArticleVersion, VersionContent)> {
        connection
            .run(move |connection| {
                return sql_query(format!(r#"
                    WITH max_versions AS (
                        SELECT article_language_id, MAX(version) AS max_version
                        FROM article_version
                        WHERE article_language_id = ANY($1::integer[])
                        and enabled = true
                        GROUP BY article_language_id
                    )
                    SELECT article_version.*, version_content.*
                    FROM article_version
                    INNER JOIN max_versions mv ON article_version.article_language_id = mv.article_language_id
                    INNER JOIN version_content ON article_version.content_id = version_content.id
                    WHERE (
                        article_version.version = mv.max_version
                        OR (
                        article_version.version > mv.max_version
                          AND article_version.enabled = false
                        )
                      )
                    ORDER BY article_version.version DESC
                    ;"#,
                ))
                .bind::<diesel::sql_types::Array<diesel::sql_types::Integer>, _>(&article_languages_ids)
                .load::<(model::ArticleVersion, VersionContent)>(connection);
            })
            .await
            .expect(&FmtError::FailedToFetch("article_versions").fmt())
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

                if query_dto.article_languages_ids.len() == 1 {
                    query = query.filter(
                        db_schema::article_version::article_language_id
                            .eq(query_dto.article_languages_ids[0]),
                    );
                } else {
                    query = query.filter(
                        db_schema::article_version::article_language_id
                            .eq_any(query_dto.article_languages_ids.clone()),
                    );
                }

                return query
                    .filter(db_schema::article_version::version.ge(query_dto.version_ge))
                    .order(db_schema::article_version::version.desc())
                    .load::<(model::ArticleVersion, VersionContent)>(connection);
            })
            .await
            .expect(&FmtError::FailedToFetch("article_versions").fmt())
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
                enabled: true,

                updated_at: None,
                created_at: None,

                updated_by: None,
                created_by: creation_dto.user_id,
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
                        enabled: patch_dto.enabled,
                        updated_by: patch_dto.user_id,

                        id: None,
                        content_id: None,
                        version: None,
                        article_language_id: None,
                        updated_at: None,
                        created_at: None,
                        created_by: None,
                    })
                    .execute(connection)
            })
            .await
            .expect(&FmtError::FailedToUpdate("article_version").fmt())
    }
}
