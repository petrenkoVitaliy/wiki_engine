use rocket_sync_db_pools::{database, diesel};

#[database("diesel")]
pub struct PgConnection(diesel::pg::PgConnection);

pub async fn wrap_db<T: Send + 'static, U: Send + 'static>(
    connection: &PgConnection,
    cb: fn(connection: &mut diesel::PgConnection, dto: U) -> Result<T, diesel::result::Error>,
    dto: U,
    error_msg: &str,
) -> T {
    connection
        .run(move |connection| cb(connection, dto))
        .await
        .expect(error_msg)
}
