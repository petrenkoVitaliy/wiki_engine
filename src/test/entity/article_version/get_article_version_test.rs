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
    article_version::request_body::{ArticleVersionCreateRelationsBody, ArticleVersionPatchBody},
};
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn get_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article version"),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let creation_body = ArticleVersionCreateRelationsBody {
        content: String::from("second version content"),
    };

    ArticleVersionRequestHandler::create_article_version(
        &setup,
        &creation_body,
        article.id,
        &language,
        admin_token,
    )
    .await;

    let first_article_version =
        ArticleVersionRequestHandler::get_article_version(&setup, article.id, &language, 1).await;

    let second_article_version =
        ArticleVersionRequestHandler::get_article_version(&setup, article.id, &language, 2).await;

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(
                &ArticleVersionCreateRelationsBody {
                    content: article_creation_body.content,
                },
                1,
            ),
        ),
        ArticleVersionAssertOptions { is_updated: false },
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &second_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&creation_body, 2),
        ),
        ArticleVersionAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn get_article_version_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article version wrong params"),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };
    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token).await;

    let wrong_language = String::from("en");

    let response_wrong_language =
        ArticleVersionRequest::get_article_version(&setup, article.id, &wrong_language, 1).await;

    assert_eq!(response_wrong_language.status(), Status::NotFound);

    let error_message = response_wrong_language.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());

    let response_wrong_version =
        ArticleVersionRequest::get_article_version(&setup, article.id, &language, 2).await;

    assert_eq!(response_wrong_version.status(), Status::NotFound);

    let error_message = response_wrong_version.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_version").fmt());
}

#[tokio::test]
async fn get_article_version_disabled() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article versions disabled"),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &language,
        1,
        admin_token.clone(),
    )
    .await;

    let response_disabled =
        ArticleVersionRequest::get_article_version(&setup, article.id, &language, 1).await;

    assert_eq!(response_disabled.status(), Status::NotFound);

    let error_message = response_disabled.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_version").fmt());

    ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: true },
        article.id,
        &language,
        1,
        admin_token,
    )
    .await;

    let response_enabled =
        ArticleVersionRequest::get_article_version(&setup, article.id, &language, 1).await;

    assert_eq!(response_enabled.status(), Status::Ok);
}
