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
    article::ArticleCreateRelationsBody, article_language::ArticleLanguagePatchBody,
};

#[tokio::test]
async fn patch_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test patch article language"),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let patched_name = String::from("patched name");

    let article_language_response = ArticleLanguageRequestHandler::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: Some(patched_name.clone()),
        },
        article.id,
        &language,
        admin_token,
    )
    .await;

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &article_language_response,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions {
                name: patched_name,
                enabled: false,

                archived: false,
                content: article_creation_body.content,
                language,
            },
        ),
        ArticleLanguageAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn patch_article_language_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: String::from("test patch article language wrong params"),
        content: String::from("test content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let patched_name = String::from("patched name");

    let wrong_article_id = 0;
    let article_language_response = ArticleLanguageRequest::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: Some(patched_name.clone()),
        },
        wrong_article_id,
        &language,
        admin_token.clone(),
    )
    .await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    let article_language_response = ArticleLanguageRequest::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: Some(patched_name.clone()),
        },
        article.id,
        &String::from("incorrect"),
        admin_token,
    )
    .await;

    assert_eq!(article_language_response.status(), Status::NotFound);

    let error_message = article_language_response.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("language").fmt());
}
