use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use diesel_derive_enum;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::{sql_types, version_content};

#[derive(Serialize, Deserialize, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "sql_types::ContentType"]
pub enum ContentType {
    Full,
    Diff,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = version_content)]
pub struct VersionContentInsertable {
    pub id: Option<i32>,

    pub content: Vec<u8>,
    pub content_type: ContentType,
    pub content_length: i32,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = version_content)]
pub struct VersionContentPatch {
    pub id: Option<i32>,

    pub content: Option<Vec<u8>>,
    pub content_type: Option<ContentType>,
    pub content_length: Option<i32>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
#[diesel(table_name = version_content)]
pub struct VersionContent {
    pub id: i32,

    pub content: Vec<u8>,

    pub content_type: ContentType,

    pub content_length: i32,
}
