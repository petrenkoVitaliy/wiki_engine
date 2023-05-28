use rocket::{response::status, serde::json::Json, *};

use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::connection;
use super::option_config::query_options::QueryOptions;
use crate::error::formatted_error::FmtError;

use super::schema::article::{
    ArticleAggregation, ArticleCreateRelationsDto, ArticlePatchBody, ArticlePatchDto,
};

use super::service::article::ArticleService;

#[openapi]
#[get("/")]
async fn get_articles(connection: connection::PgConnection) -> Json<Vec<ArticleAggregation>> {
    let articles =
        ArticleService::get_aggregations(&connection, QueryOptions { is_actual: true }).await;

    Json(articles)
}

#[openapi]
#[get("/<id>")]
async fn get_article(
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
    let article =
        ArticleService::get_aggregation(&connection, id, QueryOptions { is_actual: true }).await;

    match article {
        None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
        Some(article) => Ok(Json(article)),
    }
}

#[openapi]
#[post("/", data = "<creation_dto>")]
async fn create_article(
    connection: connection::PgConnection,
    creation_dto: Json<ArticleCreateRelationsDto>,
) -> Result<Json<ArticleAggregation>, status::BadRequest<String>> {
    let article = ArticleService::insert(
        &connection,
        ArticleCreateRelationsDto::from_json(creation_dto),
    )
    .await;

    match article {
        None => {
            return Err(status::BadRequest(Some(
                FmtError::NotFound("article").fmt(),
            )))
        }
        Some(article) => Ok(Json(article)),
    }
}

#[openapi]
#[patch("/<id>", data = "<patch_body>")]
async fn patch_article(
    connection: connection::PgConnection,
    id: i32,
    patch_body: Json<ArticlePatchBody>,
) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
    let article = ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: Some(patch_body.enabled),
            archived: None,
        },
    )
    .await;

    match article {
        None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
        Some(article) => Ok(Json(article)),
    }
}

#[openapi]
#[delete("/<id>")]
async fn delete_article(
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
    let article = ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: None,
            archived: Some(true),
        },
    )
    .await;

    match article {
        None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
        Some(article) => Ok(Json(article)),
    }
}

#[openapi]
#[post("/<id>/restore")]
async fn restore_article(
    connection: connection::PgConnection,
    id: i32,
) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
    let article = ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: None,
            archived: Some(false),
        },
    )
    .await;

    match article {
        None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
        Some(article) => Ok(Json(article)),
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
        restore_article
    ]
}
