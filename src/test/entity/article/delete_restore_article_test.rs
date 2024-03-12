use rocket::http::Status;

use super::error::FmtError;
use super::setup::{SetupOptions, TestSetup, TestUser};

use super::test_handler::{
    assert_handler::article::{ArticleAssertHandler, ArticleAssertOptions},
    mock_handler::article::{ArticleMockHandler, ArticleMockOptions},
    request_handler::article::{ArticleRequest, ArticleRequestHandler},
};

use super::dtm::article::request_body::ArticleCreateRelationsBody;
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn delete_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    let deleted_article =
        ArticleRequestHandler::delete_article(&setup, created_article.id, admin_token).await;

    ArticleAssertHandler::assert_article_aggregation(
        deleted_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            archived: true,

            enabled: true,
            name: String::from(creation_body.name),
            content: String::from(creation_body.content),
            language: String::from(creation_body.language),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn delete_nonexisting_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let response = ArticleRequest::delete_article(&setup, 0, admin_token).await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());
}

#[tokio::test]
async fn restore_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    let deleted_article =
        ArticleRequestHandler::delete_article(&setup, created_article.id, admin_token.clone())
            .await;

    ArticleAssertHandler::assert_article_aggregation(
        deleted_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            archived: true,

            enabled: true,
            name: String::from(creation_body.name.clone()),
            content: String::from(creation_body.content.clone()),
            language: String::from(creation_body.language.clone()),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );

    let restored_article =
        ArticleRequestHandler::restore_article(&setup, created_article.id, admin_token).await;

    ArticleAssertHandler::assert_article_aggregation(
        restored_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            archived: false,

            enabled: true,
            name: String::from(creation_body.name),
            content: String::from(creation_body.content),
            language: String::from(creation_body.language),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn restore_nonexisting_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    let deleted_article =
        ArticleRequestHandler::delete_article(&setup, created_article.id, admin_token.clone())
            .await;

    ArticleAssertHandler::assert_article_aggregation(
        deleted_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            archived: true,

            enabled: true,
            name: String::from(creation_body.name.clone()),
            content: String::from(creation_body.content.clone()),
            language: String::from(creation_body.language.clone()),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );

    let nonexisting_article_id = 0;
    let restored_article_response =
        ArticleRequest::restore_article(&setup, nonexisting_article_id, admin_token).await;

    assert_eq!(restored_article_response.status(), Status::NotFound);
    let error_message = restored_article_response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());
}
