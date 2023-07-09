use rocket::local::blocking::LocalResponse;
use rocket::{http::Status, uri};
use serde::Serialize;

use super::setup::TestSetup;

use super::aggregation::article_version::ArticleVersionAggregation;

use super::schema::article_version::ArticleVersionPatchBody;

use super::router::article_version::*;

pub struct ArticleVersionRequestHandler;
impl ArticleVersionRequestHandler {
    pub fn create_article_version<T>(
        setup: &TestSetup,
        creation_body: &T,
        article_id: i32,
        language_code: &String,
    ) -> ArticleVersionAggregation
    where
        T: Serialize,
    {
        let response = ArticleVersionRequest::create_article_version(
            setup,
            creation_body,
            article_id,
            language_code,
        );

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleVersionAggregation>().unwrap()
    }

    pub fn get_article_version<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        version: i32,
    ) -> ArticleVersionAggregation {
        let response =
            ArticleVersionRequest::get_article_version(setup, article_id, language_code, version);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleVersionAggregation>().unwrap()
    }

    pub fn get_article_versions<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> Vec<ArticleVersionAggregation> {
        let response =
            ArticleVersionRequest::get_article_versions(setup, article_id, language_code);

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<Vec<ArticleVersionAggregation>>()
            .unwrap()
    }

    pub fn patch_article_language<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleVersionPatchBody,
        article_id: i32,
        language_code: &String,
        version: i32,
    ) -> ArticleVersionAggregation {
        let response = ArticleVersionRequest::patch_article_version(
            setup,
            patch_body,
            article_id,
            language_code,
            version,
        );

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleVersionAggregation>().unwrap()
    }
}

pub struct ArticleVersionRequest;
impl ArticleVersionRequest {
    pub fn create_article_version<'s, T>(
        setup: &'s TestSetup,
        creation_body: &T,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .post(uri!(
                "/articles",
                create_article_version(article_id, language_code)
            ))
            .json::<T>(creation_body)
            .dispatch()
    }

    pub fn get_article_version<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        version: i32,
    ) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!(
                "/articles",
                get_article_version(article_id, language_code, version)
            ))
            .dispatch()
    }

    pub fn get_article_versions<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!(
                "/articles",
                get_article_versions(article_id, language_code,)
            ))
            .dispatch()
    }

    pub fn patch_article_version<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleVersionPatchBody,
        article_id: i32,
        language_code: &String,
        version: i32,
    ) -> LocalResponse<'s> {
        setup
            .client
            .patch(uri!(
                "/articles",
                patch_article_version(article_id, language_code, version)
            ))
            .json::<ArticleVersionPatchBody>(patch_body)
            .dispatch()
    }
}
