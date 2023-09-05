use rocket::{delete, get, patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::authorization::Authorization;
use super::dtm_common::{QueryOptions, UserRoleId};
use super::repository::PgConnection;
use super::trait_common::DtoConvert;

use super::aggregation::article_language::ArticleLanguageAggregation;
use super::dtm::article_language::{
    dto::ArticleLanguagePatchDto,
    request_body::{ArticleLanguageCreateRelationsBody, ArticleLanguagePatchBody},
};

use super::service::article_language::ArticleLanguageService;

#[openapi]
#[get("/<article_id>/language/<language_code>")]
async fn get_article_language(
    connection: PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::Custom<String>> {
    match ArticleLanguageService::get_aggregation(
        &connection,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await
    {
        Ok(article_language_aggregation) => Ok(Json(article_language_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[get("/<article_id>/language")]
async fn get_article_languages(
    connection: PgConnection,
    article_id: i32,
) -> Result<Json<Vec<ArticleLanguageAggregation>>, status::Custom<String>> {
    let article_languages = ArticleLanguageService::get_aggregations(
        &connection,
        article_id,
        &QueryOptions { is_actual: true },
    )
    .await;

    Ok(Json(article_languages))
}

#[openapi]
#[post("/<article_id>/language/<language_code>", data = "<creation_body>")]
async fn create_article_language(
    connection: PgConnection,
    authorization: Authorization,
    creation_body: Json<ArticleLanguageCreateRelationsBody>,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::Custom<String>> {
    let user_aggregation = authorization.verify(vec![], &connection).await?;

    match ArticleLanguageService::insert(
        &connection,
        creation_body
            .0
            .into_dto((user_aggregation.id, article_id, language_code)),
    )
    .await
    {
        Ok(article_language_aggregation) => Ok(Json(article_language_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[patch("/<article_id>/language/<language_code>", data = "<patch_body>")]
async fn patch_article_language(
    connection: PgConnection,
    authorization: Authorization,
    patch_body: Json<ArticleLanguagePatchBody>,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::Custom<String>> {
    let get_allowed_roles = || {
        if patch_body.enabled.is_some() {
            return vec![UserRoleId::Admin, UserRoleId::Moderator];
        }

        vec![]
    };

    let user_aggregation = authorization
        .verify(get_allowed_roles(), &connection)
        .await?;

    match ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        patch_body.0.into_dto(user_aggregation.id),
    )
    .await
    {
        Ok(article_language_aggregation) => Ok(Json(article_language_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[delete("/<article_id>/language/<language_code>")]
async fn delete_article_language(
    connection: PgConnection,
    authorization: Authorization,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::Custom<String>> {
    let user_aggregation = authorization.verify(vec![], &connection).await?;

    match ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        ArticleLanguagePatchDto {
            archived: Some(true),
            enabled: None,
            name: None,
            user_id: user_aggregation.id,
        },
    )
    .await
    {
        Ok(article_language_aggregation) => Ok(Json(article_language_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/<article_id>/language/<language_code>/restore")]
async fn restore_article_language(
    connection: PgConnection,
    authorization: Authorization,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::Custom<String>> {
    let user_aggregation = authorization
        .verify(vec![UserRoleId::Admin, UserRoleId::Moderator], &connection)
        .await?;

    match ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        ArticleLanguagePatchDto {
            archived: Some(false),
            enabled: None,
            name: None,
            user_id: user_aggregation.id,
        },
    )
    .await
    {
        Ok(article_language_aggregation) => Ok(Json(article_language_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/article_language.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings: get_article_language,
        create_article_language,
        patch_article_language,
        delete_article_language,
        restore_article_language,
        get_article_languages
    ]
}
