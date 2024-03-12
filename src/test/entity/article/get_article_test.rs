use rocket::http::Status;

use super::error::FmtError;

use super::setup::{SetupOptions, TestSetup, TestUser};

use super::test_handler::{
    assert_handler::article::{ArticleAssertHandler, ArticleAssertOptions},
    mock_handler::article::{ArticleMockHandler, ArticleMockOptions},
    request_handler::article::{ArticleRequest, ArticleRequestHandler},
};

use super::dtm::article::request_body::{ArticleCreateRelationsBody, ArticlePatchBody};
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn get_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token).await;

    let received_article = ArticleRequestHandler::get_article(&setup, created_article.id).await;

    ArticleAssertHandler::assert_article_aggregation(
        received_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn get_nonexisting_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;

    let response = ArticleRequest::get_article(&setup, 0).await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());
}

#[tokio::test]
async fn get_disabled_article() {
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

    ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody {
            enabled: Some(false),
            article_type: None,
        },
        admin_token.clone(),
    )
    .await;

    let response = ArticleRequest::get_article(&setup, created_article.id).await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());

    ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody {
            enabled: Some(true),
            article_type: None,
        },
        admin_token,
    )
    .await;

    let received_article = ArticleRequestHandler::get_article(&setup, created_article.id).await;

    ArticleAssertHandler::assert_article_aggregation(
        received_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn get_deleted_article() {
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

    ArticleRequestHandler::delete_article(&setup, created_article.id, admin_token.clone()).await;

    let response = ArticleRequest::get_article(&setup, created_article.id).await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());

    ArticleRequestHandler::restore_article(&setup, created_article.id, admin_token).await;

    let received_article = ArticleRequestHandler::get_article(&setup, created_article.id).await;

    ArticleAssertHandler::assert_article_aggregation(
        received_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: true },
    );
}
