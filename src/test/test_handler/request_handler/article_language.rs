use rocket::local::asynchronous::LocalResponse;
use rocket::{http::Status, uri};

use super::router::article_language::*;

use super::aggregation::article_language::ArticleLanguageAggregation;

use super::request_handler::RequestHandler;

use super::schema::article_language::{ArticleLanguageCreateBody, ArticleLanguagePatchBody};

use super::setup::TestSetup;

pub struct ArticleLanguageRequestHandler;
impl ArticleLanguageRequestHandler {
    pub async fn create_article_language<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::create_article_language(
            setup,
            creation_body,
            article_id,
            language_code,
            jwt_token,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<ArticleLanguageAggregation>()
            .await
            .unwrap()
    }

    pub async fn get_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
    ) -> ArticleLanguageAggregation {
        let response =
            ArticleLanguageRequest::get_article_language(setup, article_id, language_code).await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<ArticleLanguageAggregation>()
            .await
            .unwrap()
    }

    pub async fn get_article_languages<'s>(
        setup: &'s TestSetup,
        article_id: i32,
    ) -> Vec<ArticleLanguageAggregation> {
        let response = ArticleLanguageRequest::get_article_languages(setup, article_id).await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<Vec<ArticleLanguageAggregation>>()
            .await
            .unwrap()
    }

    pub async fn patch_article_language<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleLanguagePatchBody,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::patch_article_language(
            setup,
            patch_body,
            article_id,
            language_code,
            jwt_token,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<ArticleLanguageAggregation>()
            .await
            .unwrap()
    }

    pub async fn delete_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::delete_article_language(
            setup,
            article_id,
            language_code,
            jwt_token,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<ArticleLanguageAggregation>()
            .await
            .unwrap()
    }

    pub async fn restore_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> ArticleLanguageAggregation {
        let response = ArticleLanguageRequest::restore_article_language(
            setup,
            article_id,
            language_code,
            jwt_token,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);

        response
            .into_json::<ArticleLanguageAggregation>()
            .await
            .unwrap()
    }
}

pub struct ArticleLanguageRequest;
impl ArticleLanguageRequest {
    pub async fn get_article_language<'s>(
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
            .await
    }

    pub async fn get_article_languages<'s>(
        setup: &'s TestSetup,
        article_id: i32,
    ) -> LocalResponse<'s> {
        setup
            .client
            .get(uri!("/articles", get_article_languages(article_id)))
            .dispatch()
            .await
    }

    pub async fn create_article_language<'s>(
        setup: &'s TestSetup,
        creation_body: &ArticleLanguageCreateBody,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!(
                "/articles",
                create_article_language(article_id, language_code)
            ))
            .json::<ArticleLanguageCreateBody>(creation_body)
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn patch_article_language<'s>(
        setup: &'s TestSetup,
        patch_body: &ArticleLanguagePatchBody,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .patch(uri!(
                "/articles",
                patch_article_language(article_id, language_code)
            ))
            .json::<ArticleLanguagePatchBody>(patch_body)
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn delete_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .delete(uri!(
                "/articles",
                delete_article_language(article_id, language_code)
            ))
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }

    pub async fn restore_article_language<'s>(
        setup: &'s TestSetup,
        article_id: i32,
        language_code: &String,
        jwt_token: String,
    ) -> LocalResponse<'s> {
        setup
            .client
            .post(uri!(
                "/articles",
                restore_article_language(article_id, language_code)
            ))
            .header(RequestHandler::get_auth_header(jwt_token))
            .dispatch()
            .await
    }
}
