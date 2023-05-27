use super::connection;
use super::error::formatted_error::FmtError;

pub async fn wrap_db<T: Send + 'static, U: Send + 'static>(
    connection: &connection::PgConnection,
    cb: fn(connection: &mut diesel::PgConnection, dto: U) -> Result<T, diesel::result::Error>,
    dto: U,
    error: FmtError<'_>,
) -> T {
    connection
        .run(move |connection| cb(connection, dto))
        .await
        .expect(error.fmt().as_str())
}
