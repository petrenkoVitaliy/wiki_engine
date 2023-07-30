use rocket::http::Status;

use super::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup, TestUser};
use super::test_handler::{
    assert_handler::article_language::{
        ArticleLanguageAssertHandler, ArticleLanguageAssertOptions,
    },
    mock_handler::article_language::{ArticleLanguageMockHandler, ArticleLanguageMockOptions},
    request_handler::{
        article::ArticleRequestHandler,
        article_language::{ArticleLanguageRequest, ArticleLanguageRequestHandler},
    },
};

use super::repository::entity::article::ArticleType;
use super::schema::{
    article::ArticleCreateRelationsBody, article_language::ArticleLanguageCreateBody,
};

#[tokio::test]
async fn create_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let article = ArticleRequestHandler::create_article(
        &setup,
        &ArticleCreateRelationsBody {
            name: String::from("test create article language"),
            content: String::from("test content"),
            language: String::from("ua"),
            article_type: ArticleType::Public,
        },
        admin_token.clone(),
    )
    .await;

    let language = String::from("en");
    let creation_body = ArticleLanguageCreateBody {
        name: String::from("test create article language"),
        content: String::from("test create article content"),
    };

    let response_body = ArticleLanguageRequestHandler::create_article_language(
        &setup,
        &creation_body,
        article.id,
        &language,
        admin_token,
    )
    .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &response_body,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &language),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn create_article_language_with_nonexisting_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let article = ArticleRequestHandler::create_article(
        &setup,
        &ArticleCreateRelationsBody {
            name: String::from("test create article language with nonexisting language"),
            content: String::from("test content"),
            language: String::from("ua"),
            article_type: ArticleType::Public,
        },
        admin_token.clone(),
    )
    .await;

    let language = String::from("nonexisting");
    let creation_body = ArticleLanguageCreateBody {
        name: String::from("test create article language with nonexisting language"),
        content: String::from("test create article content"),
    };

    let response = ArticleLanguageRequest::create_article_language(
        &setup,
        &creation_body,
        article.id,
        &language,
        admin_token,
    )
    .await;

    assert_eq!(response.status(), Status::NotFound);

    let error_message = response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("language").fmt());
}

#[tokio::test]
async fn create_article_language_with_already_existing_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let article = ArticleRequestHandler::create_article(
        &setup,
        &ArticleCreateRelationsBody {
            name: String::from("test create article language with already existing language"),
            content: String::from("test content"),
            language: String::from("ua"),
            article_type: ArticleType::Public,
        },
        admin_token.clone(),
    )
    .await;

    let existing_language = String::from("ua");
    let creation_body = ArticleLanguageCreateBody {
        name: String::from("test create article language with already existing language"),
        content: String::from("test create article content"),
    };

    let response = ArticleLanguageRequest::create_article_language(
        &setup,
        &creation_body,
        article.id,
        &existing_language,
        admin_token,
    )
    .await;

    assert_eq!(response.status(), Status::BadRequest);

    let error_message = response.into_string().await.unwrap();
    assert_eq!(
        error_message,
        FmtError::AlreadyExists("article_language").fmt()
    );
}
