use rocket::{response::status, serde::json::Json, *};

use super::connection;
use super::option_config::query_options::QueryOptions;
use crate::error::formatted_error::FmtError;

use super::schema::article::{
    ArticleAggregation, ArticleCreateDto, ArticlePatchBody, ArticlePatchDto,
};

use super::service::article::ArticleService;

#[get("/")]
async fn get_articles(connection: connection::PgConnection) -> Json<Vec<ArticleAggregation>> {
    let articles =
        ArticleService::get_aggregations(&connection, QueryOptions { is_actual: true }).await;

    Json(articles)
}

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

#[post("/", data = "<article_dto>")]
async fn create_article(
    connection: connection::PgConnection,
    article_dto: Json<ArticleCreateDto>,
) -> Result<Json<ArticleAggregation>, status::BadRequest<String>> {
    let article =
        ArticleService::insert(&connection, ArticleCreateDto::from_json(article_dto)).await;

    match article {
        None => {
            return Err(status::BadRequest(Some(
                FmtError::NotFound("article").fmt(),
            )))
        }
        Some(article) => Ok(Json(article)),
    }
}

#[patch("/<id>", data = "<article_patch_body>")]
async fn patch_article(
    connection: connection::PgConnection,
    id: i32,
    article_patch_body: Json<ArticlePatchBody>,
) -> Result<Json<ArticleAggregation>, status::NotFound<String>> {
    let article = ArticleService::patch(
        &connection,
        ArticlePatchDto {
            id,
            enabled: Some(article_patch_body.enabled),
            archived: None,
        },
    )
    .await;

    match article {
        None => return Err(status::NotFound(FmtError::NotFound("article").fmt())),
        Some(article) => Ok(Json(article)),
    }
}

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
    routes![
        get_articles,
        get_article,
        create_article,
        patch_article,
        delete_article,
        restore_article
    ]
}
