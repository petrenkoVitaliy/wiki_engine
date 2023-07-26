use rocket::{patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::authorization::Authorization;
use super::connection;

use super::aggregation::user_account::UserAccountAggregation;

use super::schema::auth::{UserLoginBody, UserPatchBody, UserPatchDto, UserSignupBody};
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

#[openapi]
#[post("/check")]
async fn test_jwt(
    connection: connection::PgConnection,
    authorization: Authorization,
) -> Result<Json<String>, status::Custom<String>> {
    authorization
        .verify(vec![UserRoleId::Admin, UserRoleId::Common], &connection)
        .await?;

    Ok(Json(String::from("ok")))
}

#[openapi]
#[patch("/user/<user_id>", data = "<patch_body>")]
async fn patch_user(
    connection: connection::PgConnection,
    authorization: Authorization,
    user_id: i32,
    patch_body: Json<UserPatchBody>,
) -> Result<Json<UserAccountAggregation>, status::Custom<String>> {
    let user_aggregation = authorization
        .verify(vec![UserRoleId::Admin, UserRoleId::Moderator], &connection)
        .await?;

    match AuthService::patch(
        &connection,
        UserPatchDto {
            user_id,
            updated_by: user_aggregation.id,
            active: patch_body.active,
        },
    )
    .await
    {
        Ok(user_account_aggregation) => Ok(Json(user_account_aggregation)),
        Err(e) => Err(e.custom()),
    }
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
        patch_user,
    ]
}
