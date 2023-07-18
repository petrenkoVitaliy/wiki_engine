use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

use super::error::{error_wrapper::ErrorWrapper, formatted_error::FmtError};
use super::jwt_handler::JwtHandler;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

impl<'a, 'r> OpenApiFromRequest<'a> for JWT {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorWrapper;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorWrapper> {
        fn is_valid(key: &str) -> Result<Claims, ErrorWrapper> {
            Ok(JwtHandler::decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((
                Status::Unauthorized,
                FmtError::FailedToProcess("authorization").error_wrapper(), // TODO norm error
            )),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => Outcome::Failure((Status::Unauthorized, err)),
            },
        }
    }
}
