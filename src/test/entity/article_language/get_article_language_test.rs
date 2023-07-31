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

use super::dtm::{
    article::request_body::ArticleCreateRelationsBody,
    article_language::request_body::{
        ArticleLanguageCreateRelationsBody, ArticleLanguagePatchBody,
    },
};
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn get_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let first_language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article language"),
        content: String::from("test content"),
        language: first_language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let first_article_language =
        ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &first_language)
            .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &first_article_language,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(
                &ArticleLanguageCreateRelationsBody {
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                },
                &first_language,
            ),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );

    let second_language = String::from("en");
    let creation_body = ArticleLanguageCreateRelationsBody {
        name: String::from("test get article language"),
        content: String::from("test content"),
    };

    ArticleLanguageRequestHandler::create_article_language(
        &setup,
        &creation_body,
        article.id,
        &second_language,
        admin_token,
    )
    .await;

    let response_body =
        ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &second_language)
            .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &response_body,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &second_language),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn get_article_language_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article language wrong params"),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token).await;

    let article_language_response = ArticleLanguageRequest::get_article_language(
        &setup,
        article.id,
        &String::from("incorrect"),
    )
    .await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("language").fmt());

    let wrong_article_id = 0;
    let article_language_response =
        ArticleLanguageRequest::get_article_language(&setup, wrong_article_id, &language).await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}

#[tokio::test]
async fn get_article_language_deleted() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article language deleted"),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let article_language_response =
        ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &language).await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &article_language_response,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(
                &ArticleLanguageCreateRelationsBody {
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                },
                &language,
            ),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );

    ArticleLanguageRequestHandler::delete_article_language(
        &setup,
        article.id,
        &language,
        admin_token,
    )
    .await;

    let article_language_response =
        ArticleLanguageRequest::get_article_language(&setup, article.id, &language).await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}

#[tokio::test]
async fn get_article_language_disabled() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test get article language disabled"),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let article_language_response =
        ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &language).await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &article_language_response,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(
                &ArticleLanguageCreateRelationsBody {
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                },
                &language,
            ),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );

    ArticleLanguageRequestHandler::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: None,
        },
        article.id,
        &language,
        admin_token,
    )
    .await;

    let article_language_response =
        ArticleLanguageRequest::get_article_language(&setup, article.id, &language).await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}
