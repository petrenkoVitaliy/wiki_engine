use rocket::{delete, get, patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::connection;
use super::option_config::query_options::QueryOptions;

use super::aggregation::article::ArticleAggregation;

use super::schema::article::{ArticleCreateRelationsDto, ArticlePatchBody, ArticlePatchDto};

use super::service::article::ArticleService;

#[openapi]
#[get("/")]
async fn get_articles(
    connection: connection::PgConnection,
) -> Result<Json<Vec<ArticleAggregation>>, status::Custom<String>> {
    let article_aggregation =
        ArticleService::get_aggregations(&connection, &QueryOptions { is_actual: true }).await;

    return Ok(Json(article_aggregation));
}

#[openapi]
#[get("/<id>")]
async fn get_article(
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::get_aggregation(&connection, id, &QueryOptions { is_actual: true }).await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[post("/", data = "<creation_dto>")]
async fn create_article(
    connection: connection::PgConnection,
    creation_dto: Json<ArticleCreateRelationsDto>,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::insert(
        &connection,
        ArticleCreateRelationsDto::from_json(creation_dto),
    )
    .await
    {
        Ok(article_aggregation) => Ok(Json(article_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[patch("/<id>", data = "<patch_body>")]
async fn patch_article(
    connection: connection::PgConnection,
    id: i32,
    patch_body: Json<ArticlePatchBody>,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: Some(patch_body.enabled),
            archived: None,
        },
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
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: None,
            archived: Some(true),
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
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::Custom<String>> {
    match ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: None,
            archived: Some(false),
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
