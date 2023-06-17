use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::error::formatted_error::FmtError;
use super::wrapper;

use super::schema::version_content::VersionContentDto;

pub mod model;

pub struct VersionContentRepository {}

impl VersionContentRepository {
    pub async fn get_one(
        connection: &connection::PgConnection,
        id: i32,
    ) -> Option<model::VersionContent> {
        connection
            .run(move |connection| {
                let mut query = db_schema::version_content::table.into_boxed();

                query = query.filter(db_schema::version_content::id.eq(id));

                return query.first(connection).optional();
            })
            .await
            .expect(FmtError::FailedToFetch("version_content").fmt().as_str())
    }

    pub fn get_one_raw(
        connection: &mut diesel::PgConnection,
        id: i32,
    ) -> Result<Option<model::VersionContent>, diesel::result::Error> {
        let mut query = db_schema::version_content::table.into_boxed();

        query = query.filter(db_schema::version_content::id.eq(id));

        return query.first(connection).optional();
    }

    pub fn patch_raw(
        connection: &mut diesel::PgConnection,
        id: i32,
        content: Vec<u8>,
    ) -> Result<model::VersionContent, diesel::result::Error> {
        diesel::update(db_schema::version_content::table)
            .filter(db_schema::version_content::id.eq(id))
            .set(model::VersionContentPatch {
                id: None,
                content: Some(content),
                content_type: Some(model::ContentType::Diff),
                content_length: None,
            })
            .get_result::<model::VersionContent>(connection)
    }

    pub async fn get_many(
        connection: &connection::PgConnection,
        ids: Vec<i32>,
    ) -> Vec<model::VersionContent> {
        connection
            .run(move |connection| {
                let mut query = db_schema::version_content::table.into_boxed();

                query = query.filter(db_schema::version_content::id.eq_any(ids));

                return query.load(connection);
            })
            .await
            .expect(FmtError::FailedToFetch("version_contents").fmt().as_str())
    }

    pub async fn _insert(
        connection: &connection::PgConnection,
        creation_dto: VersionContentDto,
    ) -> model::VersionContent {
        wrapper::_wrap_db(
            &connection,
            Self::insert_raw,
            creation_dto,
            FmtError::FailedToInsert("version_content"),
        )
        .await
    }

    pub fn insert_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: VersionContentDto,
    ) -> Result<model::VersionContent, diesel::result::Error> {
        let content_length = creation_dto.content.len() as i32;

        diesel::insert_into(db_schema::version_content::table)
            .values(model::VersionContentInsertable {
                id: None,

                content: creation_dto.content,
                content_type: creation_dto.content_type,
                content_length,
            })
            .get_result::<model::VersionContent>(connection)
    }
}
