use rocket::local::blocking::LocalResponse;
use rocket::{http::Status, uri};
use serde::Serialize;

use super::router::article::*;

use super::aggregation::article::ArticleAggregation;

use super::setup::TestSetup;

pub struct ArticleRequestHandler;
impl ArticleRequestHandler {
    pub fn create_article<T>(setup: &TestSetup, creation_body: &T) -> ArticleAggregation
    where
        T: Serialize,
    {
        let response = ArticleRequest::create_article(setup, creation_body);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }

    pub fn get_article(setup: &TestSetup, article_id: i32) -> ArticleAggregation {
        let response = ArticleRequest::get_article(setup, article_id);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }

    pub fn get_articles(setup: &TestSetup) -> Vec<ArticleAggregation> {
        let response = ArticleRequest::get_articles(setup);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<Vec<ArticleAggregation>>().unwrap()
    }

    pub fn delete_article(setup: &TestSetup, article_id: i32) -> ArticleAggregation {
        let response = ArticleRequest::delete_article(setup, article_id);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }

    pub fn restore_article(setup: &TestSetup, article_id: i32) -> ArticleAggregation {
        let response = ArticleRequest::restore_article(setup, article_id);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }

    pub fn patch_article<T>(
        setup: &TestSetup,
        article_id: i32,
        patch_body: &T,
    ) -> ArticleAggregation
    where
        T: Serialize,
    {
        let response = ArticleRequest::patch_article(setup, article_id, patch_body);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }
}

pub struct ArticleRequest;
impl ArticleRequest {
    pub fn create_article<'s, T>(setup: &'s TestSetup, creation_body: &T) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .post(uri!("/articles", create_article))
            .json::<T>(creation_body)
            .dispatch()
    }

    pub fn patch_article<'s, T>(
        setup: &'s TestSetup,
        article_id: i32,
        patch_body: &T,
    ) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .patch(uri!("/articles", patch_article(article_id)))
            .json::<T>(patch_body)
            .dispatch()
    }

    pub fn delete_article<'s>(setup: &'s TestSetup, article_id: i32) -> LocalResponse<'s> {
        setup
            .client
            .delete(uri!("/articles", delete_article(article_id)))
            .dispatch()
    }

    pub fn restore_article<'s>(setup: &'s TestSetup, article_id: i32) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!("/articles", restore_article(article_id)))
            .dispatch()
    }

    pub fn get_article<'s>(setup: &'s TestSetup, article_id: i32) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!("/articles", get_article(article_id)))
            .dispatch()
    }

    pub fn get_articles<'s>(setup: &'s TestSetup) -> LocalResponse<'s> {
        setup.client.get(uri!("/articles", get_articles)).dispatch()
    }
}
