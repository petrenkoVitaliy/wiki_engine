use rocket::{response::status, serde::json::Json, *};

use super::connection;
use super::option_config::query_options::QueryOptions;
use crate::error::formatted_error::FmtError;

use super::schema::article_language::{
    ArticleLanguageAggregation, ArticleLanguageCreateBody, ArticleLanguageCreateRelationsDto,
};

use super::service::article_language::ArticleLanguageService;

#[get("/<article_id>/article-language/<language_code>")]
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

#[post(
    "/<article_id>/article-language/<language_code>",
    data = "<creation_body>"
)]
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
            name: creation_body.content.to_string(),
        },
    )
    .await;

    Ok(Json(article_language))
}

// #[patch("/<id>", data = "<article_patch_body>")]
// async fn patch_article(
//     connection: connection::PgConnection,
//     id: i32,
//     article_patch_body: Json<ArticlePatchBody>,
// ) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
//     let article = ArticleService::patch(
//         &connection,
//         ArticlePatchDto {
//             id,
//             enabled: Some(article_patch_body.enabled),
//             archived: None,
//         },
//     )
//     .await;

//     match article {
//         None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
//         Some(article) => Ok(Json(article)),
//     }
// }

// #[delete("/<id>")]
// async fn delete_article(
//     connection: connection::PgConnection,
//     id: i32,
// ) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
//     let article = ArticleService::patch(
//         &connection,
//         ArticlePatchDto {
//             id,
//             enabled: None,
//             archived: Some(true),
//         },
//     )
//     .await;

//     match article {
//         None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
//         Some(article) => Ok(Json(article)),
//     }
// }

// #[post("/<id>/restore")]
// async fn restore_article(
//     connection: connection::PgConnection,
//     id: i32,
// ) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
//     let article = ArticleService::patch(
//         &connection,
//         ArticlePatchDto {
//             id,
//             enabled: None,
//             archived: Some(false),
//         },
//     )
//     .await;

//     match article {
//         None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
//         Some(article) => Ok(Json(article)),
//     }
// }

pub fn routes() -> Vec<rocket::Route> {
    routes![get_article_language, create_article_language]
}
