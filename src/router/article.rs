use rocket::{delete, get, patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::authorization::Authorization;
use super::dtm_common::{QueryOptions, UserRoleId};
use super::repository::PgConnection;
use super::trait_common::DtoConvert;

use super::aggregation::article::ArticleAggregation;
use super::dtm::article::{
    dto::ArticlePatchDto,
    request_body::{ArticleCreateRelationsBody, ArticlePatchBody},
};

use super::service::article::ArticleService;

#[openapi]
#[get("/")]
async fn get_articles(
    connection: PgConnection,
) -> Result<Json<Vec<ArticleAggregation>>, status::Custom<String>> {
    let article_aggregation =
        ArticleService::get_aggregations(&connection, &QueryOptions { is_actual: true }).await;

    return Ok(Json(article_aggregation));
}

#[openapi]
#[get("/key/<article_language_key>", rank = 1)]
async fn get_aggregation_by_key(
    connection: PgConnection,
    article_language_key: String,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::get_aggregation_by_key(
        &connection,
        article_language_key,
        &QueryOptions { is_actual: true },
    )
    .await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[get("/<id>")]
async fn get_article(
    connection: PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::get_aggregation(&connection, id, &QueryOptions { is_actual: true }).await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/", data = "<creation_body>")]
async fn create_article(
    connection: PgConnection,
    authorization: Authorization,
    creation_body: Json<ArticleCreateRelationsBody>,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    let user_aggregation = authorization.verify(vec![], &connection).await?;

    match ArticleService::insert(&connection, creation_body.0.into_dto(user_aggregation.id)).await {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[patch("/<id>", data = "<patch_body>")]
async fn patch_article(
    connection: PgConnection,
    authorization: Authorization,
    id: i32,
    patch_body: Json<ArticlePatchBody>,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    let get_allowed_roles = || {
        if patch_body.enabled.is_some() {
            return vec![UserRoleId::Admin, UserRoleId::Moderator];
        }

        vec![]
    };

    let user_aggregation = authorization
        .verify(get_allowed_roles(), &connection)
        .await?;

    match ArticleService::patch(
        &connection,
        patch_body.0.into_dto((id, user_aggregation.id)),
        &user_aggregation,
    )
    .await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[delete("/<id>")]
async fn delete_article(
    connection: PgConnection,
    authorization: Authorization,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    let user_aggregation = authorization.verify(vec![], &connection).await?;

    match ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            archived: Some(true),
            user_id: user_aggregation.id,
            enabled: None,
            article_type: None,
        },
        &user_aggregation,
    )
    .await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/<id>/restore")]
async fn restore_article(
    connection: PgConnection,
    authorization: Authorization,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    let user_aggregation = authorization
        .verify(vec![UserRoleId::Moderator, UserRoleId::Admin], &connection)
        .await?;

    match ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            archived: Some(false),
            user_id: user_aggregation.id,
            enabled: None,
            article_type: None,
        },
        &user_aggregation,
    )
    .await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/article.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings: get_articles,
        get_aggregation_by_key,
        get_article,
        create_article,
        patch_article,
        delete_article,
        restore_article,
    ]
}
