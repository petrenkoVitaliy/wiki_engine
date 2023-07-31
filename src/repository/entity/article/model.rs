use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum;

use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_sync_db_pools::diesel;

use super::db_schema::{article, sql_types};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    diesel_derive_enum::DbEnum,
    JsonSchema,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[ExistingTypePath = "sql_types::ArticleType"]
pub enum ArticleType {
    Private,
    Public,
    Protected,
    Restricted,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = article)]
pub struct ArticleInsertable {
    pub id: Option<i32>,

    pub enabled: bool,
    pub archived: bool,
    pub article_type: ArticleType,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,

    pub updated_by: Option<i32>,
    pub created_by: i32,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = article)]
pub struct ArticlePatch {
    pub id: Option<i32>,

    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub article_type: Option<ArticleType>,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,

    pub updated_by: i32,
    pub created_by: Option<i32>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = article)]
pub struct Article {
    pub id: i32,

    pub enabled: bool,
    pub archived: bool,
    pub article_type: ArticleType,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub updated_by: Option<i32>,
    pub created_by: i32,
}
