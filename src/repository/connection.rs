use rocket_sync_db_pools::{database, diesel};

#[database("diesel")]
pub struct PgConnection(diesel::pg::PgConnection);
