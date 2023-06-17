use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel;

use super::db_schema::language;

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = language)]
pub struct Language {
    pub id: i32,
    pub code: String,
}
