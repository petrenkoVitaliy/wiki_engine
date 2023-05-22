use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::article;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = article)]
pub struct ArticleInsertable {
    pub id: Option<i32>,
    pub enabled: Option<bool>,
    pub archived: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = article)]
pub struct Article {
    pub id: i32,
    pub enabled: bool,
    pub archived: bool,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
