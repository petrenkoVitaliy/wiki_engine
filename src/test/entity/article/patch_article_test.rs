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
async fn patch_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: String::from("test patch article"),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    let patched_article = ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody { enabled: false },
        admin_token.clone(),
    )
    .await;

    ArticleAssertHandler::assert_article_aggregation(
        patched_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            enabled: false,

            archived: false,
            name: String::from(creation_body.name.clone()),
            content: String::from(creation_body.content.clone()),
            language: String::from(creation_body.language.clone()),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );

    let patched_article = ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody { enabled: true },
        admin_token,
    )
    .await;

    ArticleAssertHandler::assert_article_aggregation(
        patched_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
            enabled: true,

            archived: false,
            name: String::from(creation_body.name.clone()),
            content: String::from(creation_body.content.clone()),
            language: String::from(creation_body.language.clone()),
            article_type: ArticleType::Public,
        }),
        ArticleAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn patch_nonexisting_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let response =
        ArticleRequest::patch_article(&setup, 0, &ArticlePatchBody { enabled: false }, admin_token)
            .await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article").fmt());
}
