use rocket::{get, patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::authorization::Authorization;
use super::dtm_common::{TokenDto, UserRoleId};
use super::repository::PgConnection;
use super::trait_common::DtoConvert;

use super::aggregation::user_account::UserAccountAggregation;
use super::aggregation::user_account_auth::UserAccountAuthAggregation;
use super::dtm::auth::request_body::{UserLoginBody, UserPatchBody, UserSignupBody};

use super::service::auth::AuthService;

#[openapi]
#[post("/signup", data = "<user_signup_body>")]
async fn signup(
    connection: PgConnection,
    user_signup_body: Json<UserSignupBody>,
) -> Result<Json<UserAccountAggregation>, status::Custom<String>> {
    match AuthService::create_user(&connection, user_signup_body.0.into_dto(())).await {
        Ok(user_account_aggregation) => Ok(Json(user_account_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/signup/role/<role_id>", data = "<user_signup_body>")]
#[allow(dead_code)] // tests
async fn signup_with_role(
    connection: PgConnection,
    user_signup_body: Json<UserSignupBody>,
    role_id: i32,
) -> Result<Json<TokenDto>, status::Custom<String>> {
    match AuthService::create_user_with_role(&connection, user_signup_body.0.into_dto(()), role_id)
        .await
    {
        Ok(token_response) => Ok(Json(token_response)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/login", data = "<user_login_body>")]
async fn login(
    connection: PgConnection,
    user_login_body: Json<UserLoginBody>,
) -> Result<Json<UserAccountAuthAggregation>, status::Custom<String>> {
    match AuthService::login(&connection, user_login_body.0.into_dto(())).await {
        Ok(aggregation) => Ok(Json(aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[get("/user")]
async fn get_user(
    connection: PgConnection,
    authorization: Authorization,
) -> Result<Json<UserAccountAggregation>, status::Custom<String>> {
    let user_aggregation = authorization.verify(vec![], &connection).await?;

    Ok(Json(user_aggregation))
}

#[openapi]
#[post("/check")]
async fn test_jwt(
    connection: PgConnection,
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
    connection: PgConnection,
    authorization: Authorization,
    user_id: i32,
    patch_body: Json<UserPatchBody>,
) -> Result<Json<UserAccountAggregation>, status::Custom<String>> {
    let user_aggregation = authorization
        .verify(vec![UserRoleId::Admin, UserRoleId::Moderator], &connection)
        .await?;

    match AuthService::patch(
        &connection,
        patch_body.0.into_dto((user_id, user_aggregation.id)),
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

    openapi_get_routes![settings: signup, login, test_jwt, patch_user, get_user]
}

#[allow(dead_code)] // tests
pub fn test_routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/auth.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings: signup,
        login,
        test_jwt,
        patch_user,
        signup_with_role,
        get_user,
    ]
}
