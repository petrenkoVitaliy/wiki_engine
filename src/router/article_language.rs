use rocket::{response::status, serde::json::Json, *};

use rocket_okapi::{
    okapi::schemars::gen::SchemaSettings, openapi, openapi_get_routes, settings::OpenApiSettings,
};

use super::connection;
use super::option_config::query_options::QueryOptions;
use crate::error::formatted_error::FmtError;

use super::schema::article_language::{
    ArticleLanguageAggregation, ArticleLanguageCreateBody, ArticleLanguageCreateRelationsDto,
    ArticleLanguagePatchBody, ArticleLanguagePatchDto,
};

use super::service::article_language::ArticleLanguageService;

#[openapi]
#[get("/<article_id>/language/<language_code>")]
async fn get_article_language(
    connection: connection::PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::NotFound<String>> {
    let article_language = ArticleLanguageService::get_aggregation(
        &connection,
        article_id,
        language_code,
        QueryOptions { is_actual: true },
    )
    .await;

    match article_language {
        None => {
            return Err(status::NotFound(
                FmtError::NotFound("article_language").fmt(),
            ))
        }
        Some(article_language) => Ok(Json(article_language)),
    }
}

#[openapi]
#[get("/<article_id>/language")]
async fn get_article_languages(
    connection: connection::PgConnection,
    article_id: i32,
) -> Result<Json<Vec<ArticleLanguageAggregation>>, status::NotFound<String>> {
    let article_language = ArticleLanguageService::get_aggregations(
        &connection,
        vec![article_id],
        QueryOptions { is_actual: true },
    )
    .await;

    Ok(Json(article_language))
}

#[openapi]
#[post("/<article_id>/language/<language_code>", data = "<creation_body>")]
async fn create_article_language(
    connection: connection::PgConnection,
    creation_body: Json<ArticleLanguageCreateBody>,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::BadRequest<String>> {
    let article_language = ArticleLanguageService::insert(
        &connection,
        ArticleLanguageCreateRelationsDto {
            article_id,
            language_code,
            content: creation_body.content.to_string(),
            name: creation_body.name.to_string(),
        },
    )
    .await;

    Ok(Json(article_language))
}

#[openapi]
#[patch("/<article_id>/language/<language_code>", data = "<patch_body>")]
async fn patch_article_language(
    connection: connection::PgConnection,
    patch_body: Json<ArticleLanguagePatchBody>,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::NotFound<String>> {
    let article_language = ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        ArticleLanguagePatchDto {
            enabled: patch_body.enabled,
            name: match patch_body.name.as_ref() {
                Some(name) => Some(String::from(name)),
                _ => None,
            },
            archived: None,
        },
    )
    .await;

    match article_language {
        Some(article_language) => Ok(Json(article_language)),
        _ => {
            return Err(status::NotFound(
                FmtError::NotFound("article_language").fmt(),
            ))
        }
    }
}

#[openapi]
#[delete("/<article_id>/language/<language_code>")]
async fn delete_article_language(
    connection: connection::PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::NotFound<String>> {
    let article_language = ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        ArticleLanguagePatchDto {
            archived: Some(true),
            enabled: None,
            name: None,
        },
    )
    .await;

    match article_language {
        Some(article_language) => Ok(Json(article_language)),
        _ => {
            return Err(status::NotFound(
                FmtError::NotFound("article_language").fmt(),
            ))
        }
    }
}

#[openapi]
#[post("/<article_id>/language/<language_code>/restore")]
async fn restore_article_language(
    connection: connection::PgConnection,
    article_id: i32,
    language_code: String,
) -> Result<Json<ArticleLanguageAggregation>, status::NotFound<String>> {
    let article_language = ArticleLanguageService::patch(
        &connection,
        language_code,
        article_id,
        ArticleLanguagePatchDto {
            archived: Some(false),
            enabled: None,
            name: None,
        },
    )
    .await;

    match article_language {
        Some(article_language) => Ok(Json(article_language)),
        _ => {
            return Err(status::NotFound(
                FmtError::NotFound("article_language").fmt(),
            ))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    let settings = OpenApiSettings {
        json_path: "/article_language.json".to_owned(),
        schema_settings: SchemaSettings::openapi3(),
    };

    openapi_get_routes![
        settings:
        get_article_language,
        create_article_language,
        patch_article_language,
        delete_article_language,
        restore_article_language,
        get_article_languages
    ]
}