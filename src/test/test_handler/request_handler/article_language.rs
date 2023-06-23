use rocket::local::blocking::LocalResponse;
use rocket::{http::Status, uri};

use super::router::article_language::*;

use super::aggregation::article_language::ArticleLanguageAggregation;
use super::schema::article_language::ArticleLanguageCreateBody;
use super::setup::TestSetup;

pub struct ArticleLanguageRequestHandler;
impl ArticleLanguageRequestHandler {
    pub fn create_article_language_handler<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::create_article_language(
            setup,
            creation_body,
            article_id,
            language_code,
        );

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleLanguageAggregation>().unwrap()
    }
}

pub struct ArticleLanguageRequest;
impl ArticleLanguageRequest {
    pub fn create_article_language<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!(
                "/articles",
                create_article_language(article_id, language_code)
            ))
            .json::<ArticleLanguageCreateBody>(creation_body)
            .dispatch()
    }
}
