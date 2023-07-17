use rocket::http::Status;

use super::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup};
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

use super::schema::{
    article::ArticleCreateRelationsDto,
    article_language::{ArticleLanguageCreateBody, ArticleLanguagePatchBody},
};

mod create_article_language_tests {
    use super::*;

    #[test]
    fn create_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let article = ArticleRequestHandler::create_article(
            &setup,
            &ArticleCreateRelationsDto {
                name: String::from("test create article language"),
                content: String::from("test content"),
                language: String::from("ua"),
            },
        );

        let language = String::from("en");
        let creation_body = ArticleLanguageCreateBody {
            name: String::from("test create article language"),
            content: String::from("test create article content"),
        };

        let response_body = ArticleLanguageRequestHandler::create_article_language(
            &setup,
            &creation_body,
            article.id,
            &language,
        );

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &response_body,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &language),
            ),
            ArticleLanguageAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn create_article_language_with_nonexisting_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let article = ArticleRequestHandler::create_article(
            &setup,
            &ArticleCreateRelationsDto {
                name: String::from("test create article language with nonexisting language"),
                content: String::from("test content"),
                language: String::from("ua"),
            },
        );

        let language = String::from("nonexisting");
        let creation_body = ArticleLanguageCreateBody {
            name: String::from("test create article language with nonexisting language"),
            content: String::from("test create article content"),
        };

        let response = ArticleLanguageRequest::create_article_language(
            &setup,
            &creation_body,
            article.id,
            &language,
        );

        assert_eq!(response.status(), Status::NotFound);

        let error_message = response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("language").fmt());
    }

    #[test]
    fn create_article_language_with_already_existing_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let article = ArticleRequestHandler::create_article(
            &setup,
            &ArticleCreateRelationsDto {
                name: String::from("test create article language with already existing language"),
                content: String::from("test content"),
                language: String::from("ua"),
            },
        );

        let existing_language = String::from("ua");
        let creation_body = ArticleLanguageCreateBody {
            name: String::from("test create article language with already existing language"),
            content: String::from("test create article content"),
        };

        let response = ArticleLanguageRequest::create_article_language(
            &setup,
            &creation_body,
            article.id,
            &existing_language,
        );

        assert_eq!(response.status(), Status::BadRequest);

        let error_message = response.into_string().unwrap();
        assert_eq!(
            error_message,
            FmtError::AlreadyExists("article_language").fmt()
        );
    }
}

mod get_article_language_tests {
    use super::*;

    #[test]
    fn get_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let first_language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language"),
            content: String::from("test content"),
            language: first_language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let first_article_language = ArticleLanguageRequestHandler::get_article_language(
            &setup,
            article.id,
            &first_language,
        );

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &first_article_language,
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
        );

        let response_body = ArticleLanguageRequestHandler::get_article_language(
            &setup,
            article.id,
            &second_language,
        );

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &response_body,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions::from_creation_dto(&creation_body, &second_language),
            ),
            ArticleLanguageAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn get_article_language_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language wrong params"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let article_language_response = ArticleLanguageRequest::get_article_language(
            &setup,
            article.id,
            &String::from("incorrect"),
        );

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("language").fmt());

        let wrong_article_id = 0;
        let article_language_response =
            ArticleLanguageRequest::get_article_language(&setup, wrong_article_id, &language);

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }

    #[test]
    fn get_article_language_deleted() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language deleted"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let article_language_response =
            ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &language);

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &article_language_response,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions::from_creation_dto(
                    &ArticleLanguageCreateBody {
                        name: article_creation_body.name,
                        content: article_creation_body.content,
                    },
                    &language,
                ),
            ),
            ArticleLanguageAssertOptions { is_updated: false },
        );

        ArticleLanguageRequestHandler::delete_article_language(&setup, article.id, &language);

        let article_language_response =
            ArticleLanguageRequest::get_article_language(&setup, article.id, &language);

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }

    #[test]
    fn get_article_language_disabled() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language disabled"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let article_language_response =
            ArticleLanguageRequestHandler::get_article_language(&setup, article.id, &language);

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &article_language_response,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions::from_creation_dto(
                    &ArticleLanguageCreateBody {
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
        );

        let article_language_response =
            ArticleLanguageRequest::get_article_language(&setup, article.id, &language);

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }
}

mod get_article_languages_tests {
    use super::*;

    #[test]
    fn get_article_languages() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let first_language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language"),
            content: String::from("test content"),
            language: first_language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);
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
        );

        let response_body =
            ArticleLanguageRequestHandler::get_article_languages(&setup, article.id);

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

    #[test]
    fn get_article_languages_disabled() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let first_language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article language"),
            content: String::from("test content"),
            language: first_language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

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
        );

        ArticleLanguageRequestHandler::patch_article_language(
            &setup,
            &ArticleLanguagePatchBody {
                enabled: Some(false),
                name: None,
            },
            article.id,
            &first_language,
        );

        let response_body =
            ArticleLanguageRequestHandler::get_article_languages(&setup, article.id);

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
        );

        ArticleLanguageRequestHandler::patch_article_language(
            &setup,
            &ArticleLanguagePatchBody {
                enabled: Some(false),
                name: None,
            },
            article.id,
            &second_language,
        );

        let response_body =
            ArticleLanguageRequestHandler::get_article_languages(&setup, article.id);

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
}

mod patch_article_language_tests {
    use super::*;

    #[test]
    fn patch_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test patch article language"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let patched_name = String::from("patched name");

        let article_language_response = ArticleLanguageRequestHandler::patch_article_language(
            &setup,
            &ArticleLanguagePatchBody {
                enabled: Some(false),
                name: Some(patched_name.clone()),
            },
            article.id,
            &language,
        );

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

    #[test]
    fn patch_article_language_wrong_params() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("test patch article language wrong params"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

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
        );

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
        let article_language_response = ArticleLanguageRequest::patch_article_language(
            &setup,
            &ArticleLanguagePatchBody {
                enabled: Some(false),
                name: Some(patched_name.clone()),
            },
            article.id,
            &String::from("incorrect"),
        );

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("language").fmt());
    }
}

mod delete_restore_article_language_tests {
    use super::*;

    #[test]
    fn delete_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("delete article language"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let article_language_response =
            ArticleLanguageRequestHandler::delete_article_language(&setup, article.id, &language);

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &article_language_response,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions {
                    archived: true,

                    enabled: true,
                    name: article_creation_body.name,
                    content: article_creation_body.content,
                    language,
                },
            ),
            ArticleLanguageAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn delete_nonexisting_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("delete article language"),
            content: String::from("test content"),
            language: language.clone(),
        };

        ArticleRequestHandler::create_article(&setup, &article_creation_body);

        let nonexisting_article_id = 0;
        let article_language_response = ArticleLanguageRequest::delete_article_language(
            &setup,
            nonexisting_article_id,
            &language,
        );

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }

    #[test]
    fn restore_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("restore article language"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        ArticleLanguageRequestHandler::delete_article_language(&setup, article.id, &language);

        let article_language_response =
            ArticleLanguageRequestHandler::restore_article_language(&setup, article.id, &language);

        ArticleLanguageAssertHandler::assert_article_languages_aggregation(
            &article_language_response,
            &ArticleLanguageMockHandler::get_article_language_aggregation(
                &ArticleLanguageMockOptions {
                    archived: false,
                    enabled: true,

                    name: article_creation_body.name,
                    content: article_creation_body.content,
                    language,
                },
            ),
            ArticleLanguageAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn restore_nonexisting_article_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let language = String::from("ua");
        let article_creation_body = ArticleCreateRelationsDto {
            name: String::from("restore article language"),
            content: String::from("test content"),
            language: language.clone(),
        };

        let article = ArticleRequestHandler::create_article(&setup, &article_creation_body);

        ArticleLanguageRequestHandler::delete_article_language(&setup, article.id, &language);

        let nonexisting_article_id = 0;
        let article_language_response = ArticleLanguageRequest::restore_article_language(
            &setup,
            nonexisting_article_id,
            &language,
        );

        assert_eq!(article_language_response.status(), Status::NotFound);

        let error_message = article_language_response.into_string().unwrap();
        assert_eq!(error_message, FmtError::NotFound("article_language").fmt());
    }
}
