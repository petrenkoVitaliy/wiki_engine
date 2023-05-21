use rocket::{response::status, serde::json::Json, *};

use super::connection;

use super::schema::article::{ArticleAggregation, CreateArticleDto};

use super::service::article::ArticleService;

#[get("/")]
async fn get_articles(connection: connection::PgConnection) -> Json<Vec<ArticleAggregation>> {
    let articles = ArticleService::get_aggregations(&connection).await;

    Json(articles)
}

#[get("/<id>")]
async fn get_article(connection: connection::PgConnection, id: i32) -> Json<ArticleAggregation> {
    let article = ArticleService::get_aggregation(&connection, id).await;

    Json(article)
}

#[post("/", data = "<article_dto>")]
async fn create_article(
    connection: connection::PgConnection,
    article_dto: Json<CreateArticleDto>,
) -> Result<Json<ArticleAggregation>, status::BadRequest<&'static str>> {
    let article = ArticleService::insert(&connection, article_dto).await;

    match article {
        None => return Err(status::BadRequest(Some("incorrect body"))),
        Some(article) => Ok(Json(article)),
    }
}

#[put("/<id>", data = "<_article_dto>")]
fn update_article(id: u8, _article_dto: Json<CreateArticleDto>) -> String {
    format!("update article {}++", id)
}

#[patch("/<id>", data = "<_article_dto>")]
fn patch_article(id: u8, _article_dto: Json<CreateArticleDto>) -> String {
    format!("patch article {}", id)
}

#[delete("/<id>")]
fn delete_article(id: u8) -> String {
    format!("delete article {}", id)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_articles,
        get_article,
        create_article,
        update_article,
        patch_article,
        delete_article
    ]
}
