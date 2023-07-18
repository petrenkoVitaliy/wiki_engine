use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

use super::error::{error_wrapper::ErrorWrapper, formatted_error::FmtError};
use super::jwt_handler::JwtHandler;
use super::schema::jwt::Claims;
use super::schema::user_role::UserRoleId;

#[derive(Debug)]
pub struct Authorization {
    pub token: Option<String>,
}

impl Authorization {
    pub fn verify(self, allowed_roles: Vec<UserRoleId>) -> Result<Claims, status::Custom<String>> {
        let claims = match Self::get_claims(self) {
            Err(e) => return Err(e.custom()),
            Ok(claims) => claims,
        };

        if allowed_roles.len() == 0 {
            return Ok(claims);
        }

        match allowed_roles
            .iter()
            .find(|role_id| match UserRoleId::from_i32(claims.role_id) {
                Some(current_user_role_id) => current_user_role_id == **role_id,
                _ => false,
            }) {
            Some(_) => Ok(claims),
            _ => Err(FmtError::PermissionDenied("not enough rights")
                .error_wrapper()
                .custom()),
        }
    }

    fn get_claims(self) -> Result<Claims, ErrorWrapper> {
        match self.token {
            None => FmtError::Unauthorized("empty authorization").error(),
            Some(token) => match JwtHandler::decode_jwt(token) {
                Ok(claims) => Ok(claims),
                Err(err) => {
                    let error_wrapper = match err {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            FmtError::Unauthorized("expired token").error_wrapper()
                        }
                        jsonwebtoken::errors::ErrorKind::InvalidToken => {
                            FmtError::Unauthorized("invalid token").error_wrapper()
                        }
                        _ => FmtError::Unauthorized("unknown token")
                            .error_wrapper_enriched(format!("{:?}", err)),
                    };

                    Err(error_wrapper)
                }
            },
        }
    }
}

impl<'a, 'r> OpenApiFromRequest<'a> for Authorization {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ErrorWrapper;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorWrapper> {
        return Outcome::Success(Self {
            token: match req.headers().get_one("authorization") {
                Some(token) => Some(String::from(token)),
                _ => None,
            },
        });
    }
}
