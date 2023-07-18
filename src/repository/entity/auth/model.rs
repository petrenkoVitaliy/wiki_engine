use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::user_account;
use super::db_schema::user_password;

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user_account)]
pub struct UserAccount {
    pub id: i32,

    pub email: String,
    pub name: String,

    pub role_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = user_account)]
pub struct UserAccountInsertable {
    pub id: Option<i32>,

    pub email: String,
    pub name: String,

    pub role_id: i32,

    pub updated_at: Option<Option<NaiveDateTime>>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = user_password)]
pub struct UserPassword {
    pub id: i32,

    pub password: String,
    pub user_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = user_password)]
pub struct UserPasswordInsertable {
    pub id: Option<i32>,

    pub password: String,
    pub user_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}
