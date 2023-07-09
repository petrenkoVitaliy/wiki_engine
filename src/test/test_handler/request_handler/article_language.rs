use rocket::local::blocking::LocalResponse;
use rocket::{http::Status, uri};

use super::router::article_language::*;

use super::aggregation::article_language::ArticleLanguageAggregation;

use super::schema::article_language::{ArticleLanguageCreateBody, ArticleLanguagePatchBody};

use super::setup::TestSetup;

pub struct ArticleLanguageRequestHandler;
impl ArticleLanguageRequestHandler {
    pub fn create_article_language<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: &String,
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

    pub fn get_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> ArticleLanguageAggregation {
        let response =
            ArticleLanguageRequest::get_article_language(setup, article_id, language_code);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleLanguageAggregation>().unwrap()
    }

    pub fn get_article_languages<'s>(
        setup: &'s TestSetup,
        article_id: i32,
    ) -> Vec<ArticleLanguageAggregation> {
        let response = ArticleLanguageRequest::get_article_languages(setup, article_id);

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<Vec<ArticleLanguageAggregation>>()
            .unwrap()
    }

    pub fn patch_article_language<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleLanguagePatchBody,
        article_id: i32,
        language_code: &String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::patch_article_language(
            setup,
            patch_body,
            article_id,
            language_code,
        );

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleLanguageAggregation>().unwrap()
    }

    pub fn delete_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> ArticleLanguageAggregation {
        let response =
            ArticleLanguageRequest::delete_article_language(setup, article_id, language_code);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleLanguageAggregation>().unwrap()
    }

    pub fn restore_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> ArticleLanguageAggregation {
        let response =
            ArticleLanguageRequest::restore_article_language(setup, article_id, language_code);

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<ArticleLanguageAggregation>().unwrap()
    }
}

pub struct ArticleLanguageRequest;
impl ArticleLanguageRequest {
    pub fn get_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!(
                "/articles",
                get_article_language(article_id, language_code)
            ))
            .dispatch()
    }

    pub fn get_article_languages<'s>(setup: &'s TestSetup, article_id: i32) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!("/articles", get_article_languages(article_id)))
            .dispatch()
    }

    pub fn create_article_language<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: &String,
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

    pub fn patch_article_language<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleLanguagePatchBody,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .patch(uri!(
                "/articles",
                patch_article_language(article_id, language_code)
            ))
            .json::<ArticleLanguagePatchBody>(patch_body)
            .dispatch()
    }

    pub fn delete_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .delete(uri!(
                "/articles",
                delete_article_language(article_id, language_code)
            ))
            .dispatch()
    }

    pub fn restore_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!(
                "/articles",
                restore_article_language(article_id, language_code)
            ))
            .dispatch()
    }
}
