use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserSignupBody {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl UserSignupBody {
    pub fn from_json(json_body: Json<Self>) -> Self {
        Self {
            email: json_body.email.to_string(),
            name: json_body.name.to_string(),
            password: json_body.password.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserLoginBody {
    pub email: String,
    pub password: String,
}

impl UserLoginBody {
    pub fn from_json(json_body: Json<Self>) -> Self {
        Self {
            email: json_body.email.to_string(),
            password: json_body.password.to_string(),
        }
    }
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct UserAccountCreateDto {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct UserPasswordCreateDto {
    pub user_id: i32,
    pub password_hash: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct UserCreateRelationsDto {
    pub email: String,
    pub name: String,
    pub password_hash: String,
}