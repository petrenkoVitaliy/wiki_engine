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
async fn get_article_versions() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };

    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let creation_body = ArticleVersionCreateRelationsBody {
        content: String::from("second version content"),
        name: None,
    };

    ArticleVersionRequestHandler::create_article_version(
        &setup,
        &creation_body,
        article.id,
        &language,
        admin_token,
    )
    .await;

    let article_versions =
        ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language).await;

    let (first_article_version, second_article_version) =
        (&article_versions[1], &article_versions[0]);

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(
                &ArticleVersionCreateRelationsBody {
                    content: article_creation_body.content,
                    name: None,
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
async fn get_article_versions_wrong_params() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };
    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let wrong_language = String::from("en");

    let response_wrong_language =
        ArticleVersionRequest::get_article_versions(&setup, article.id, &wrong_language).await;

    assert_eq!(response_wrong_language.status(), Status::NotFound);

    let error_message = response_wrong_language.into_string().await.unwrap();
    assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
}

#[tokio::test]
async fn get_article_versions_disabled() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("first version content"),
        language: language.clone(),
        article_type: ArticleType::Public,
    };
    let article =
        ArticleRequestHandler::create_article(&setup, &article_creation_body, admin_token.clone())
            .await;

    let creation_body = ArticleVersionCreateRelationsBody {
        content: String::from("second version content"),
        name: None,
    };

    ArticleVersionRequestHandler::create_article_version(
        &setup,
        &creation_body,
        article.id,
        &language,
        admin_token.clone(),
    )
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

    let article_versions =
        ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language).await;

    assert_eq!(article_versions.len(), 1);

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &article_versions[0],
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&creation_body, 2),
        ),
        ArticleVersionAssertOptions { is_updated: false },
    );

    ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: true },
        article.id,
        &language,
        1,
        admin_token.clone(),
    )
    .await;

    ArticleVersionRequestHandler::patch_article_language(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &language,
        2,
        admin_token,
    )
    .await;

    let article_versions =
        ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language).await;

    assert_eq!(article_versions.len(), 1);

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &article_versions[0],
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(
                &ArticleVersionCreateRelationsBody {
                    content: article_creation_body.content,
                    name: None,
                },
                1,
            ),
        ),
        ArticleVersionAssertOptions { is_updated: true },
    );
}
