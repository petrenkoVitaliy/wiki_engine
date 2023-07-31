use rocket::http::Status;

use super::error::FmtError;

use super::setup::{SetupOptions, TestSetup, TestUser};
use super::test_handler::{
    assert_handler::article_version::{ArticleVersionAssertHandler, ArticleVersionAssertOptions},
    mock_handler::article_version::{ArticleVersionMockHandler, ArticleVersionMockOptions},
    request_handler::{
        article::ArticleRequestHandler,
        article_version::{ArticleVersionRequest, ArticleVersionRequestHandler},
    },
};

use super::dtm::{
    article::request_body::ArticleCreateRelationsBody,
    article_version::request_body::ArticleVersionPatchBody,
};
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn patch_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token: String = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article versions"),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let patched_article_version = ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &language,
        1,
        admin_token.clone(),
    )
    .await;

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &patched_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(&ArticleVersionMockOptions {
            content: article_creation_body.content.clone(),
            version: 1,
            enabled: false,
        }),
        ArticleVersionAssertOptions { is_updated: true },
    );

    let patched_article_version = ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: true },
        article.id,
        &language,
        1,
        admin_token,
    )
    .await;

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &patched_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(&ArticleVersionMockOptions {
            content: article_creation_body.content.clone(),
            version: 1,
            enabled: true,
        }),
        ArticleVersionAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn patch_article_version_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token: String = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test patch article version wrong params"),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };
    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let wrong_language = String::from("en");

    let response_wrong_language = ArticleVersionRequest::patch_article_version(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &wrong_language,
        1,
        admin_token.clone(),
    )
    .await;

    assert_eq!(response_wrong_language.status(), Status::NotFound);

    let error_message = response_wrong_language.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());

    let response_wrong_version = ArticleVersionRequest::patch_article_version(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &language,
        2,
        admin_token,
    )
    .await;

    assert_eq!(response_wrong_version.status(), Status::NotFound);

    let error_message = response_wrong_version.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_version").fmt());
}
