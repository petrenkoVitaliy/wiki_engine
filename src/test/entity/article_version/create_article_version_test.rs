use rocket::http::Status;

use super::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup, TestUser};
use super::test_handler::{
    assert_handler::article_version::{ArticleVersionAssertHandler, ArticleVersionAssertOptions},
    mock_handler::article_version::{ArticleVersionMockHandler, ArticleVersionMockOptions},
    request_handler::{
        article::ArticleRequestHandler,
        article_version::{ArticleVersionRequest, ArticleVersionRequestHandler},
    },
};

use super::repository::entity::article::ArticleType;
use super::schema::{
    article::ArticleCreateRelationsBody, article_version::ArticleVersionCreateBody,
};

#[tokio::test]
async fn create_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article = ArticleRequestHandler::create_article(
        &setup,
        &ArticleCreateRelationsBody {
            name: String::from("test create article version"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        },
        admin_token.clone(),
    )
    .await;

    let first_creation_body = ArticleVersionCreateBody {
        content: String::from("second version content"),
    };

    let first_response_body = ArticleVersionRequestHandler::create_article_version(
        &setup,
        &first_creation_body,
        article.id,
        &language,
        admin_token.clone(),
    )
    .await;

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_response_body,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&first_creation_body, 2),
        ),
        ArticleVersionAssertOptions { is_updated: false },
    );

    let second_creation_body = ArticleVersionCreateBody {
        content: String::from("test create article version content"),
    };

    let second_response_body = ArticleVersionRequestHandler::create_article_version(
        &setup,
        &second_creation_body,
        article.id,
        &language,
        admin_token,
    )
    .await;

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &second_response_body,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&second_creation_body, 3),
        ),
        ArticleVersionAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn create_article_version_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article = ArticleRequestHandler::create_article(
        &setup,
        &ArticleCreateRelationsBody {
            name: String::from("test create article version wrong params"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        },
        admin_token.clone(),
    )
    .await;

    let first_creation_body = ArticleVersionCreateBody {
        content: String::from("second version content"),
    };

    let wrong_language = String::from("en");

    let article_version_wrong_language = ArticleVersionRequest::create_article_version(
        &setup,
        &first_creation_body,
        article.id,
        &wrong_language,
        admin_token,
    )
    .await;

    assert_eq!(article_version_wrong_language.status(), Status::NotFound);

    let error_message = article_version_wrong_language.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}
