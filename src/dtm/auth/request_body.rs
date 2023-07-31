use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::trait_common::DtoConvert;

use super::dto::{UserLoginDto, UserPatchDto, UserSignupDto};

#[derive(Deserialize, JsonSchema)]
pub struct UserPatchBody {
    pub active: bool,
}

impl DtoConvert<UserPatchDto> for UserPatchBody {
    type TParams = (i32, i32);

    fn into_dto(self, (user_id, updated_by): Self::TParams) -> UserPatchDto {
        UserPatchDto {
            user_id,
            updated_by,
            active: self.active,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserSignupBody {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl DtoConvert<UserSignupDto> for UserSignupBody {
    type TParams = ();

    fn into_dto(self, _params: Self::TParams) -> UserSignupDto {
        UserSignupDto {
            email: self.email,
            name: self.name,
            password: self.password,
        }
    }
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct UserLoginBody {
    pub email: String,
    pub password: String,
}

impl DtoConvert<UserLoginDto> for UserLoginBody {
    type TParams = ();

    fn into_dto(self, _params: Self::TParams) -> UserLoginDto {
        UserLoginDto {
            email: self.email,
            password: self.password,
        }
    }
}
