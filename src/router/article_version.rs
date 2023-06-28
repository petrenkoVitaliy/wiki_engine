use rocket::{get, patch, post, response::status, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::connection;
use super::option_config::query_options::QueryOptions;

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
) -> Result<Json<ArticleVersionAggregation>, status::Custom<String>> {
    match ArticleVersionService::get_aggregation(
        &connection,
        id,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await
    {
        Ok(article_version_aggregation) => Ok(Json(article_version_aggregation)),
        Err(e) => Err(e.custom()),
    }
}

#[openapi]
#[get("/<article_id>/language/<language_code>/version")]
async fn get_article_versions(
    connection: connection::PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<Vec<ArticleVersionAggregation>>, status::Custom<String>> {
    match ArticleVersionService::get_aggregations(
        &connection,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await
    {
        Err(e) => Err(e.custom()),
        Ok(article_versions_aggregations) => Ok(Json(article_versions_aggregations)),
    }
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
) -> Result<Json<ArticleVersionAggregation>, status::Custom<String>> {
    match ArticleVersionService::insert(
        &connection,
        article_id,
        language_code,
        ArticleVersionCreateBody {
            content: creation_body.content.to_string(),
        },
    )
    .await
    {
        Ok(article_version_aggregation) => Ok(Json(article_version_aggregation)),
        Err(e) => Err(e.custom()),
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
) -> Result<Json<ArticleVersionAggregation>, status::Custom<String>> {
    match ArticleVersionService::patch(
        &connection,
        id,
        article_id,
        language_code,
        ArticleVersionPatchBody {
            enabled: patch_body.enabled,
        },
    )
    .await
    {
        Ok(article_version_aggregation) => Ok(Json(article_version_aggregation)),
        Err(e) => Err(e.custom()),
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
