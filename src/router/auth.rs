use rocket::{post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::authorization::Authorization;
use super::connection;

use super::aggregation::user_account::UserAccountAggregation;

use super::schema::auth::{UserLoginBody, UserSignupBody};
use super::schema::jwt::TokenResponse;
use super::schema::user_role::UserRoleId;

use super::service::auth::AuthService;

#[openapi]
#[post("/signup", data = "<user_signup_body>")]
async fn signup(
    connection: connection::PgConnection,
    user_signup_body: Json<UserSignupBody>,
) -> Result<Json<UserAccountAggregation>, status::Custom<String>> {
    match AuthService::create_user(&connection, UserSignupBody::from_json(user_signup_body)).await {
        Ok(user_account_aggregation) => Ok(Json(user_account_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/login", data = "<user_login_body>")]
async fn login(
    connection: connection::PgConnection,
    user_login_body: Json<UserLoginBody>,
) -> Result<Json<TokenResponse>, status::Custom<String>> {
    match AuthService::login(&connection, UserLoginBody::from_json(user_login_body)).await {
        Ok(token_response) => Ok(Json(token_response)),
        Err(e) => Err(e.custom()),
    }
}

// TODO rm
#[openapi]
#[post("/test")]
async fn test_jwt(authorization: Authorization) -> Result<Json<String>, status::Custom<String>> {
    authorization.verify(vec![UserRoleId::Admin, UserRoleId::Common])?;

    Ok(Json(String::from("ok")))
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/auth.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings:
        signup,
        login,
        test_jwt,
    ]
}
