use rocket_sync_db_pools::{database, diesel};

use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
    Result,
};

#[database("diesel")]
pub struct PgConnection(diesel::pg::PgConnection);

impl<'a, 'r> OpenApiFromRequest<'a> for PgConnection {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
