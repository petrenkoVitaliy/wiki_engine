use rocket::http::Status;

use super::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup};
use super::test_handler::{
    assert_handler::article_version::{ArticleVersionAssertHandler, ArticleVersionAssertOptions},
    mock_handler::article_version::{ArticleVersionMockHandler, ArticleVersionMockOptions},
    request_handler::{
        article::ArticleRequestHandler,
        article_version::{ArticleVersionRequest, ArticleVersionRequestHandler},
    },
};

use super::schema::{
    article::ArticleCreateRelationsDto,
    article_version::{ArticleVersionCreateBody, ArticleVersionPatchBody},
};
use super::ArticleType;

mod get_article_versions_tests {
    use super::*;

    #[test]
    fn get_article_versions() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article versions"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let creation_body = ArticleVersionCreateBody {
            content: String::from("second version content"),
        };

        ArticleVersionRequestHandler::create_article_version(
            &setup,
            &creation_body,
            article.id,
            &language,
        );

        let article_versions =
            ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language);

        let (first_article_version, second_article_version) =
            (&article_versions[1], &article_versions[0]);

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

    #[test]
    fn get_article_versions_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article versions wrong params"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };
        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let wrong_language = String::from("en");

        let response_wrong_language =
            ArticleVersionRequest::get_article_versions(&setup, article.id, &wrong_language);

        assert_eq!(response_wrong_language.status(), Status::NotFound);

        let error_message = response_wrong_language.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }

    #[test]
    fn get_article_versions_disabled() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article versions disabled"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };
        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let creation_body = ArticleVersionCreateBody {
            content: String::from("second version content"),
        };

        ArticleVersionRequestHandler::create_article_version(
            &setup,
            &creation_body,
            article.id,
            &language,
        );

        ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &language,
            1,
        );

        let article_versions =
            ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language);

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
        );

        ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &language,
            2,
        );

        let article_versions =
            ArticleVersionRequestHandler::get_article_versions(&setup, article.id, &language);

        assert_eq!(article_versions.len(), 1);

        ArticleVersionAssertHandler::assert_article_version_aggregation(
            &article_versions[0],
            &ArticleVersionMockHandler::get_article_version_aggregation(
                &ArticleVersionMockOptions::from_creation_dto(
                    &ArticleVersionCreateBody {
                        content: article_creation_body.content,
                    },
                    1,
                ),
            ),
            ArticleVersionAssertOptions { is_updated: true },
        );
    }
}

mod patch_article_version_tests {
    use super::*;

    #[test]
    fn patch_article_version() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article versions"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let patched_article_version = ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &language,
            1,
        );

        ArticleVersionAssertHandler::assert_article_version_aggregation(
            &patched_article_version,
            &ArticleVersionMockHandler::get_article_version_aggregation(
                &ArticleVersionMockOptions {
                    content: article_creation_body.content.clone(),
                    version: 1,
                    enabled: false,
                },
            ),
            ArticleVersionAssertOptions { is_updated: true },
        );

        let patched_article_version = ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: true },
            article.id,
            &language,
            1,
        );

        ArticleVersionAssertHandler::assert_article_version_aggregation(
            &patched_article_version,
            &ArticleVersionMockHandler::get_article_version_aggregation(
                &ArticleVersionMockOptions {
                    content: article_creation_body.content.clone(),
                    version: 1,
                    enabled: true,
                },
            ),
            ArticleVersionAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn patch_article_version_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test patch article version wrong params"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };
        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let wrong_language = String::from("en");

        let response_wrong_language = ArticleVersionRequest::patch_article_version(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &wrong_language,
            1,
        );

        assert_eq!(response_wrong_language.status(), Status::NotFound);

        let error_message = response_wrong_language.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());

        let response_wrong_version = ArticleVersionRequest::patch_article_version(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &language,
            2,
        );

        assert_eq!(response_wrong_version.status(), Status::NotFound);

        let error_message = response_wrong_version.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_version").fmt());
    }
}

mod get_article_version_tests {
    use super::*;

    #[test]
    fn get_article_version() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article version"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let creation_body = ArticleVersionCreateBody {
            content: String::from("second version content"),
        };

        ArticleVersionRequestHandler::create_article_version(
            &setup,
            &creation_body,
            article.id,
            &language,
        );

        let first_article_version =
            ArticleVersionRequestHandler::get_article_version(&setup, article.id, &language, 1);

        let second_article_version =
            ArticleVersionRequestHandler::get_article_version(&setup, article.id, &language, 2);

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

    #[test]
    fn get_article_version_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article version wrong params"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };
        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let wrong_language = String::from("en");

        let response_wrong_language =
            ArticleVersionRequest::get_article_version(&setup, article.id, &wrong_language, 1);

        assert_eq!(response_wrong_language.status(), Status::NotFound);

        let error_message = response_wrong_language.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());

        let response_wrong_version =
            ArticleVersionRequest::get_article_version(&setup, article.id, &language, 2);

        assert_eq!(response_wrong_version.status(), Status::NotFound);

        let error_message = response_wrong_version.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_version").fmt());
    }

    #[test]
    fn get_article_version_disabled() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article versions disabled"),
            content: String::from("first version content"),
            language: language.clone(),
            article_type: ArticleType::Public,
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: false },
            article.id,
            &language,
            1,
        );

        let response_disabled =
            ArticleVersionRequest::get_article_version(&setup, article.id, &language, 1);

        assert_eq!(response_disabled.status(), Status::NotFound);

        let error_message = response_disabled.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_version").fmt());

        ArticleVersionRequestHandler::patch_article_language(
            &setup,
            &ArticleVersionPatchBody { enabled: true },
            article.id,
            &language,
            1,
        );

        let response_enabled =
            ArticleVersionRequest::get_article_version(&setup, article.id, &language, 1);

        assert_eq!(response_enabled.status(), Status::Ok);
    }
}

mod create_article_version_tests {
    use super::*;

    #[test]
    fn create_article_version() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article = ArticleRequestHandler::create_article(
            &setup,
            &ArticleCreateRelationsDto {
                name: String::from("test create article version"),
                content: String::from("first version content"),
                language: language.clone(),
                article_type: ArticleType::Public,
            },
        );

        let first_creation_body = ArticleVersionCreateBody {
            content: String::from("second version content"),
        };

        let first_response_body = ArticleVersionRequestHandler::create_article_version(
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
            ArticleVersionAssertOptions { is_updated: false },
        );

        let second_creation_body = ArticleVersionCreateBody {
            content: String::from("test create article version content"),
        };

        let second_response_body = ArticleVersionRequestHandler::create_article_version(
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
            ArticleVersionAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn create_article_version_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article = ArticleRequestHandler::create_article(
            &setup,
            &ArticleCreateRelationsDto {
                name: String::from("test create article version wrong params"),
                content: String::from("first version content"),
                language: language.clone(),
                article_type: ArticleType::Public,
            },
        );

        let first_creation_body = ArticleVersionCreateBody {
            content: String::from("second version content"),
        };

        let wrong_language = String::from("en");

        let article_version_wrong_language = ArticleVersionRequest::create_article_version(
            &setup,
            &first_creation_body,
            article.id,
            &wrong_language,
        );

        assert_eq!(article_version_wrong_language.status(), Status::NotFound);

        let error_message = article_version_wrong_language.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }
}
