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
    let user_aggregation = authorization
        .verify(vec![UserRoleId::Moderator, UserRoleId::Admin], &connection)
        .await?;

    match ArticleService::patch(
        &connection,
        patch_body.0.into_dto((id, user_aggregation.id)),
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
            enabled: None,
            archived: Some(true),
            user_id: user_aggregation.id,
        },
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
            enabled: None,
            archived: Some(false),
            user_id: user_aggregation.id,
        },
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
        settings:
        get_articles,
        get_article,
        create_article,
        patch_article,
        delete_article,
        restore_article,
    ]
}
