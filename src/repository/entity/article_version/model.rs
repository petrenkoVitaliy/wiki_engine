use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::article_version;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = article_version)]
pub struct ArticleVersionInsertable {
    pub id: Option<i32>,

    pub version: i32,
    pub content_id: i32,

    pub enabled: Option<bool>,

    pub article_language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = article_version)]
pub struct ArticleVersionPatch {
    pub id: Option<i32>,

    pub version: Option<i32>,
    pub content_id: Option<i32>,

    pub enabled: Option<bool>,

    pub article_language_id: Option<i32>,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
#[diesel(table_name = article_version)]
pub struct ArticleVersion {
    pub id: i32,

    pub version: i32,
    pub content_id: i32,

    pub enabled: bool,

    pub article_language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
pub struct RawIdStruct {
    #[diesel(sql_type = diesel::sql_types::Int4)]
    pub id: i32,
}
