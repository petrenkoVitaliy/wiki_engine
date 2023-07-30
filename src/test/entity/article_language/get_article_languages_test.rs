use super::setup::{SetupOptions, TestSetup, TestUser};
use super::test_handler::{
    assert_handler::article_language::{
        ArticleLanguageAssertHandler, ArticleLanguageAssertOptions,
    },
    mock_handler::article_language::{ArticleLanguageMockHandler, ArticleLanguageMockOptions},
    request_handler::{
        article::ArticleRequestHandler, article_language::ArticleLanguageRequestHandler,
    },
};

use super::repository::entity::article::ArticleType;
use super::schema::{
    article::ArticleCreateRelationsBody,
    article_language::{ArticleLanguageCreateBody, ArticleLanguagePatchBody},
};

#[tokio::test]
async fn get_article_languages() {
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
    let first_article_language = &article.languages[0];

    let second_language = String::from("en");
    let creation_body = ArticleLanguageCreateBody {
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
        ArticleLanguageRequestHandler::get_article_languages(&setup, article.id).await;

    let (first, second) = match &response_body[0].id {
        id if id == &first_article_language.id => (&response_body[0], &response_body[1]),
        _ => (&response_body[1], &response_body[0]),
    };

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        first,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(
                &ArticleLanguageCreateBody {
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                },
                &first_language,
            ),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        second,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &second_language),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn get_article_languages_disabled() {
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

    let second_language = String::from("en");
    let creation_body = ArticleLanguageCreateBody {
        name: String::from("test get article language"),
        content: String::from("test content"),
    };
    ArticleLanguageRequestHandler::create_article_language(
        &setup,
        &creation_body,
        article.id,
        &second_language,
        admin_token.clone(),
    )
    .await;

    ArticleLanguageRequestHandler::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: None,
        },
        article.id,
        &first_language,
        admin_token.clone(),
    )
    .await;

    let response_body =
        ArticleLanguageRequestHandler::get_article_languages(&setup, article.id).await;

    assert_eq!(response_body.len(), 1);

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &response_body[0],
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &second_language),
        ),
        ArticleLanguageAssertOptions { is_updated: false },
    );

    ArticleLanguageRequestHandler::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(true),
            name: None,
        },
        article.id,
        &first_language,
        admin_token.clone(),
    )
    .await;

    ArticleLanguageRequestHandler::patch_article_language(
        &setup,
        &ArticleLanguagePatchBody {
            enabled: Some(false),
            name: None,
        },
        article.id,
        &second_language,
        admin_token,
    )
    .await;

    let response_body =
        ArticleLanguageRequestHandler::get_article_languages(&setup, article.id).await;

    assert_eq!(response_body.len(), 1);

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &response_body[0],
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            &ArticleLanguageMockOptions::from_creation_dto(
                &ArticleLanguageCreateBody {
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                },
                &first_language,
            ),
        ),
        ArticleLanguageAssertOptions { is_updated: true },
    );
}
