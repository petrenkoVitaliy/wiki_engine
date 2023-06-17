use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::article_language;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = article_language)]
pub struct ArticleLanguagePatch {
    pub id: Option<i32>,
    pub name: Option<String>,

    pub enabled: Option<bool>,
    pub archived: Option<bool>,

    pub article_id: Option<i32>,
    pub language_id: Option<i32>,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = article_language)]
pub struct ArticleLanguageInsertable {
    pub id: Option<i32>,
    pub name: String,

    pub enabled: Option<bool>,
    pub archived: Option<bool>,

    pub article_id: i32,
    pub language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = article_language)]
pub struct ArticleLanguage {
    pub id: i32,
    pub name: String,

    pub enabled: bool,
    pub archived: bool,

    pub article_id: i32,
    pub language_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
