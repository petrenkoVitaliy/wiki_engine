use rocket::local::asynchronous::LocalResponse;
use rocket::{http::Status, uri};
use serde::Serialize;

use super::router::article::*;

use super::aggregation::article::ArticleAggregation;

use super::request_handler::RequestHandler;
use super::setup::TestSetup;

pub struct ArticleRequestHandler;
impl ArticleRequestHandler {
    pub async fn create_article<T>(
        setup: &TestSetup,
        creation_body: &T,
        jwt_token: String,
    ) -> ArticleAggregation
    where
        T: Serialize,
    {
        let response = ArticleRequest::create_article(setup, creation_body, jwt_token).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().await.unwrap()
    }

    pub async fn get_article(setup: &TestSetup, article_id: i32) -> ArticleAggregation {
        let response = ArticleRequest::get_article(setup, article_id).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().await.unwrap()
    }

    pub async fn get_articles(setup: &TestSetup) -> Vec<ArticleAggregation> {
        let response = ArticleRequest::get_articles(setup).await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<Vec<ArticleAggregation>>()
            .await
            .unwrap()
    }

    pub async fn delete_article(
        setup: &TestSetup,
        article_id: i32,
        jwt_token: String,
    ) -> ArticleAggregation {
        let response = ArticleRequest::delete_article(setup, article_id, jwt_token).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().await.unwrap()
    }

    pub async fn restore_article(
        setup: &TestSetup,
        article_id: i32,
        jwt_token: String,
    ) -> ArticleAggregation {
        let response = ArticleRequest::restore_article(setup, article_id, jwt_token).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().await.unwrap()
    }

    pub async fn patch_article<T>(
        setup: &TestSetup,
        article_id: i32,
        patch_body: &T,
        jwt_token: String,
    ) -> ArticleAggregation
    where
        T: Serialize,
    {
        let response =
            ArticleRequest::patch_article(setup, article_id, patch_body, jwt_token).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().await.unwrap()
    }
}

pub struct ArticleRequest;
impl ArticleRequest {
    pub async fn create_article<'s, T>(
        setup: &'s TestSetup,
        creation_body: &T,
        jwt_token: String,
    ) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .post(uri!("/articles", create_article))
            .json::<T>(creation_body)
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn patch_article<'s, T>(
        setup: &'s TestSetup,
        article_id: i32,
        patch_body: &T,
        jwt_token: String,
    ) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .patch(uri!("/articles", patch_article(article_id)))
            .json::<T>(patch_body)
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn delete_article<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .delete(uri!("/articles", delete_article(article_id)))
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn restore_article<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!("/articles", restore_article(article_id)))
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn get_article<'s>(setup: &'s TestSetup, article_id: i32) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!("/articles", get_article(article_id)))
            .dispatch()
            .await
    }

    pub async fn get_articles<'s>(setup: &'s TestSetup) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!("/articles", get_articles))
            .dispatch()
            .await
    }
}
