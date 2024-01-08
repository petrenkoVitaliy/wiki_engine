use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName, Selectable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::{user_account, user_otp, user_password};

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user_role)]
pub struct UserRole {
    pub id: i32,
    pub role: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
#[diesel(table_name = user_account)]
pub struct UserAccount {
    pub id: i32,

    pub email: String,
    pub name: String,
    pub active: bool,
    pub blocked: bool,

    pub role_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub updated_by: Option<i32>,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = user_account)]
pub struct UserAccountInsertable {
    pub id: Option<i32>,

    pub email: String,
    pub name: String,
    pub active: bool,
    pub blocked: bool,

    pub role_id: i32,

    pub updated_at: Option<Option<NaiveDateTime>>,
    pub created_at: Option<NaiveDateTime>,

    pub updated_by: Option<i32>,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = user_account)]
pub struct UserAccountPatch {
    pub id: Option<i32>,

    pub email: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
    pub blocked: Option<bool>,

    pub role_id: Option<i32>,

    pub updated_at: Option<Option<NaiveDateTime>>,
    pub created_at: Option<NaiveDateTime>,

    pub updated_by: Option<i32>,
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

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable)]
#[diesel(table_name = user_otp)]
pub struct UserOtp {
    pub id: i32,

    pub otp: String,
    pub user_id: i32,

    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = user_otp)]
pub struct UserOtpInsertable {
    pub id: Option<i32>,

    pub otp: String,
    pub user_id: i32,

    pub created_at: Option<NaiveDateTime>,
}
