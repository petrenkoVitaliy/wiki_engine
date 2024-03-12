use rocket::http::Status;

use super::error::FmtError;
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

use super::dtm::article::request_body::ArticleCreateRelationsBody;
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn delete_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let article_language_response = ArticleLanguageRequestHandler::delete_article_language(
        &setup,
        article.id,
        &language,
        admin_token,
    )
    .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &article_language_response,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions {
                archived: true,

                enabled: true,
                name: article_creation_body.name,
                content: article_creation_body.content,
                language,
            },
        ),
        ArticleLanguageAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn delete_nonexisting_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
        .await;

    let nonexisting_article_id = 0;
    let article_language_response = ArticleLanguageRequest::delete_article_language(
        &setup,
        nonexisting_article_id,
        &language,
        admin_token,
    )
    .await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}

#[tokio::test]
async fn restore_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    ArticleLanguageRequestHandler::delete_article_language(
        &setup,
        article.id,
        &language,
        admin_token.clone(),
    )
    .await;

    let article_language_response = ArticleLanguageRequestHandler::restore_article_language(
        &setup,
        article.id,
        &language,
        admin_token,
    )
    .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &article_language_response,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions {
                archived: false,
                enabled: true,

                name: article_creation_body.name,
                content: article_creation_body.content,
                language,
            },
        ),
        ArticleLanguageAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn restore_nonexisting_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    ArticleLanguageRequestHandler::delete_article_language(
        &setup,
        article.id,
        &language,
        admin_token.clone(),
    )
    .await;

    let nonexisting_article_id = 0;
    let article_language_response = ArticleLanguageRequest::restore_article_language(
        &setup,
        nonexisting_article_id,
        &language,
        admin_token,
    )
    .await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}
