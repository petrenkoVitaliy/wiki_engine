use rocket::http::Status;

use crate::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup};
use super::test_handler::{
    assert_handler::article_version::ArticleVersionAssertHandler,
    mock_handler::article_version::{ArticleVersionMockHandler, ArticleVersionMockOptions},
    request_handler::{
        article::ArticleRequestHandler, article_version::ArticleVersionRequestHandler,
    },
};

use super::schema::{
    article::ArticleCreateRelationsDto,
    article_version::{ArticleVersionCreateBody, ArticleVersionPatchBody},
};

#[test]
fn create_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let language = String::from("ua");
    let article = ArticleRequestHandler::create_article_handler(
        &setup,
        &ArticleCreateRelationsDto {
            name: String::from("test create article version"),
            content: String::from("first version content"),
            language: language.clone(),
        },
    );

    let first_creation_body = ArticleVersionCreateBody {
        content: String::from("second version content"),
    };

    let first_response_body = ArticleVersionRequestHandler::create_article_version_handler(
        &setup,
        &first_creation_body,
        article.id,
        &language,
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_response_body,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&first_creation_body, 2),
        ),
    );

    let second_creation_body = ArticleVersionCreateBody {
        content: String::from("test create article version content"),
    };

    let second_response_body = ArticleVersionRequestHandler::create_article_version_handler(
        &setup,
        &second_creation_body,
        article.id,
        &language,
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &second_response_body,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&second_creation_body, 3),
        ),
    );
}

#[test]
fn get_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsDto {
        name: String::from("test get article version"),
        content: String::from("first version content"),
        language: language.clone(),
    };

    let article = ArticleRequestHandler::create_article_handler(&setup, &article_creation_body);

    let creation_body = ArticleVersionCreateBody {
        content: String::from("second version content"),
    };

    ArticleVersionRequestHandler::create_article_version_handler(
        &setup,
        &creation_body,
        article.id,
        &language,
    );

    let first_article_version =
        ArticleVersionRequestHandler::get_article_version_handler(&setup, article.id, &language, 1);

    let second_article_version =
        ArticleVersionRequestHandler::get_article_version_handler(&setup, article.id, &language, 2);

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(
                &ArticleVersionCreateBody {
                    content: article_creation_body.content,
                },
                1,
            ),
        ),
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &second_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&creation_body, 2),
        ),
    );
}

#[test]
fn get_article_versions() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsDto {
        name: String::from("test get article versions"),
        content: String::from("first version content"),
        language: language.clone(),
    };

    let article = ArticleRequestHandler::create_article_handler(&setup, &article_creation_body);

    let creation_body = ArticleVersionCreateBody {
        content: String::from("second version content"),
    };

    ArticleVersionRequestHandler::create_article_version_handler(
        &setup,
        &creation_body,
        article.id,
        &language,
    );

    let article_versions =
        ArticleVersionRequestHandler::get_article_versions_handler(&setup, article.id, &language);

    let (first_article_version, second_article_version) =
        (&article_versions[0], &article_versions[1]);

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &first_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(
                &ArticleVersionCreateBody {
                    content: article_creation_body.content,
                },
                1,
            ),
        ),
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &second_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(
            &ArticleVersionMockOptions::from_creation_dto(&creation_body, 2),
        ),
    );
}

#[test]
fn patch_article_version() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let language = String::from("ua");
    let article_creation_body = ArticleCreateRelationsDto {
        name: String::from("test get article versions"),
        content: String::from("first version content"),
        language: language.clone(),
    };

    let article = ArticleRequestHandler::create_article_handler(&setup, &article_creation_body);

    let patched_article_version = ArticleVersionRequestHandler::patch_article_language_handler(
        &setup,
        &ArticleVersionPatchBody { enabled: false },
        article.id,
        &language,
        1,
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &patched_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(&ArticleVersionMockOptions {
            content: article_creation_body.content.clone(),
            version: 1,
            enabled: false,
        }),
    );

    let patched_article_version = ArticleVersionRequestHandler::patch_article_language_handler(
        &setup,
        &ArticleVersionPatchBody { enabled: true },
        article.id,
        &language,
        1,
    );

    ArticleVersionAssertHandler::assert_article_version_aggregation(
        &patched_article_version,
        &ArticleVersionMockHandler::get_article_version_aggregation(&ArticleVersionMockOptions {
            content: article_creation_body.content.clone(),
            version: 1,
            enabled: true,
        }),
    );
}
