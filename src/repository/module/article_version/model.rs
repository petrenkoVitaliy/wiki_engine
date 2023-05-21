use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::article_version;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = article_version)]
pub struct ArticleVersionInsertable {
    pub id: Option<i32>,

    pub version: i32,
    pub content: String,

    pub enabled: Option<bool>,

    pub article_language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = article_version)]
pub struct ArticleVersion {
    pub id: i32,
    pub version: i32,
    pub content: String,

    pub enabled: bool,

    pub article_language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
