use rocket::{response::status, serde::json::Json, *};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::connection;
use super::option_config::query_options::QueryOptions;
use crate::error::formatted_error::FmtError;

use super::schema::article_version::{ArticleVersionCreateBody, ArticleVersionPatchBody};

use super::service::article_version::ArticleVersionService;

use super::aggregation::article_version::ArticleVersionAggregation;

#[openapi]
#[get("/<article_id>/language/<language_code>/version/<id>")]
pub async fn get_article_version(
    connection: connection::PgConnection,
    article_id: i32,
    id: i32,
    language_code: String,
) -> Result<Json<ArticleVersionAggregation>, status::NotFound<String>> {
    let article_version = ArticleVersionService::get_aggregation(
        &connection,
        id,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await;

    match article_version {
        None => {
            return Err(status::NotFound(
                FmtError::NotFound("article_version").fmt(),
            ))
        }
        Some(article_version) => Ok(Json(article_version)),
    }
}

#[openapi]
#[get("/<article_id>/language/<language_code>/version")]
async fn get_article_versions(
    connection: connection::PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<Vec<ArticleVersionAggregation>>, status::NotFound<String>> {
    let article_versions = ArticleVersionService::get_aggregations(
        &connection,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await;

    Ok(Json(article_versions))
}

#[openapi]
#[post(
    "/<article_id>/language/<language_code>/version",
    data = "<creation_body>"
)]
async fn create_article_version(
    connection: connection::PgConnection,
    creation_body: Json<ArticleVersionCreateBody>,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleVersionAggregation>, status::BadRequest<String>> {
    let article_version = ArticleVersionService::insert(
        &connection,
        article_id,
        language_code,
        ArticleVersionCreateBody {
            content: creation_body.content.to_string(),
        },
    )
    .await;

    match article_version {
        Some(article_version) => Ok(Json(article_version)),
        _ => {
            return Err(status::BadRequest(Some(
                FmtError::NotFound("article_version").fmt(),
            )))
        }
    }
}

#[openapi]
#[patch(
    "/<article_id>/language/<language_code>/version/<id>",
    data = "<patch_body>"
)]
async fn patch_article_version(
    connection: connection::PgConnection,
    article_id: i32,
    id: i32,
    language_code: String,
    patch_body: Json<ArticleVersionPatchBody>,
) -> Result<Json<ArticleVersionAggregation>, status::NotFound<String>> {
    let article_version = ArticleVersionService::patch(
        &connection,
        id,
        article_id,
        language_code,
        ArticleVersionPatchBody {
            enabled: patch_body.enabled,
        },
    )
    .await;

    match article_version {
        Some(article_version) => Ok(Json(article_version)),
        _ => {
            return Err(status::NotFound(
                FmtError::NotFound("article_version").fmt(),
            ))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/article_version.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings:
        get_article_versions,
        get_article_version,
        create_article_version,
        patch_article_version
    ]
}
