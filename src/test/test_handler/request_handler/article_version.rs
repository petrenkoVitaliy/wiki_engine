use rocket::local::blocking::LocalResponse;
use rocket::{http::Status, uri};
use serde::Serialize;

use super::router::article::*;

use super::aggregation::article::ArticleAggregation;
use super::setup::TestSetup;

pub struct ArticleVersionRequestHandler;
impl ArticleVersionRequestHandler {
    pub fn create_article_handler<T>(setup: &TestSetup, creation_body: &T) -> ArticleAggregation
    where
        T: Serialize,
    {
        let response = ArticleVersionRequest::create_article(setup, creation_body);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleAggregation>().unwrap()
    }
}

pub struct ArticleVersionRequest;
impl ArticleVersionRequest {
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
}
