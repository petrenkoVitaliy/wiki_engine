use rocket::http::Status;

use super::error::formatted_error::FmtError;

use super::setup::{SetupOptions, TestSetup};

use super::test_handler::{
    assert_handler::article::{ArticleAssertHandler, ArticleAssertOptions},
    mock_handler::article::{ArticleMockHandler, ArticleMockOptions},
    request_handler::article::{ArticleRequest, ArticleRequestHandler},
};

use super::schema::article::{ArticleCreateRelationsDto, ArticlePatchBody};

mod create_article_tests {
    use super::*;

    #[test]
    fn create_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test create article"),
            content: String::from("test create article content"),
            language: String::from("ua"),
        };

        let response_body = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleAssertHandler::assert_article_aggregation(
            response_body,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn create_article_with_wrong_language() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test create wrong article"),
            content: String::from("test content"),
            language: String::from("incorrect"),
        };

        let response = ArticleRequest::create_article(&setup, &creation_body);

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("language").fmt());
    }

    #[test]
    fn create_large_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let content = "
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam et sodales ipsum, vitae imperdiet ex. 
    Vivamus at arcu libero. Nullam quam magna, condimentum eu tristique a, elementum nec ligula. 
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed id ultrices mauris. Morbi eget leo eget nisi vestibulum pulvinar. 
    Donec porta sapien in nunc ultrices, quis suscipit nisl vulputate. Cras egestas et neque non lobortis.
    Donec tempus volutpat nulla, non dictum enim maximus eget. Nunc finibus sagittis rhoncus. In hac habitasse platea dictumst. 
    Ut elementum rutrum augue, et hendrerit felis interdum tristique. Nunc vulputate accumsan tellus, vitae consequat ipsum mattis sed. 
    Morbi sit amet turpis mollis, porta turpis vitae, sagittis felis. Nullam non hendrerit sapien. Morbi sodales urna vitae volutpat fermentum.
    Cras mollis vel quam vel varius. Etiam ornare efficitur feugiat. Donec vitae nibh at turpis placerat efficitur. Nullam ut bibendum purus. 
    Proin eget est a neque mattis posuere donec.
    ";

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test name 1"),
            content: String::from(content),
            language: String::from("ua"),
        };

        let response_body = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleAssertHandler::assert_article_aggregation(
            response_body,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: false },
        );
    }
}

mod get_article_tests {
    use super::*;

    #[test]
    fn get_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test get article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let received_article = ArticleRequestHandler::get_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            received_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn get_nonexisting_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let response = ArticleRequest::get_article(&setup, 0);

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());
    }

    #[test]
    fn get_disabled_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test get disabled article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: false },
        );

        let response = ArticleRequest::get_article(&setup, created_article.id);

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());

        ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: true },
        );

        let received_article = ArticleRequestHandler::get_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            received_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn get_deleted_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test deleted article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleRequestHandler::delete_article(&setup, created_article.id);

        let response = ArticleRequest::get_article(&setup, created_article.id);

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());

        ArticleRequestHandler::restore_article(&setup, created_article.id);

        let received_article = ArticleRequestHandler::get_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            received_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: true },
        );
    }
}

mod get_articles_tests {
    use super::*;

    #[test]
    fn get_articles() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test get articles"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let received_articles = ArticleRequestHandler::get_articles(&setup);

        let expected_article = received_articles
            .into_iter()
            .find(|article| article.id == created_article.id)
            .expect("Article not found");

        assert_eq!(expected_article.languages[0].name, creation_body.name);

        ArticleAssertHandler::assert_article_aggregation(
            expected_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: false },
        );
    }

    #[test]
    fn get_disabled_articles() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test get disabled articles"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: false },
        );

        let received_articles = ArticleRequestHandler::get_articles(&setup);

        let expected_article = received_articles
            .into_iter()
            .find(|article| article.id == created_article.id);

        assert_eq!(expected_article.is_none(), true);

        ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: true },
        );

        let received_articles = ArticleRequestHandler::get_articles(&setup);

        let expected_article = received_articles
            .into_iter()
            .find(|article| article.id == created_article.id)
            .expect("Article not found");

        assert_eq!(expected_article.languages[0].name, creation_body.name);

        ArticleAssertHandler::assert_article_aggregation(
            expected_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn get_deleted_articles() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test get disabled articles"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        ArticleRequestHandler::delete_article(&setup, created_article.id);

        let received_articles = ArticleRequestHandler::get_articles(&setup);

        let expected_article = received_articles
            .into_iter()
            .find(|article| article.id == created_article.id);

        assert_eq!(expected_article.is_none(), true);

        ArticleRequestHandler::restore_article(&setup, created_article.id);

        let received_articles = ArticleRequestHandler::get_articles(&setup);

        let expected_article = received_articles
            .into_iter()
            .find(|article| article.id == created_article.id)
            .expect("Article not found");

        assert_eq!(expected_article.languages[0].name, creation_body.name);

        ArticleAssertHandler::assert_article_aggregation(
            expected_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_dto(
                creation_body,
            )),
            ArticleAssertOptions { is_updated: true },
        );
    }
}

mod patch_article_tests {
    use super::*;

    #[test]
    fn patch_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test patch article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let patched_article = ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: false },
        );

        ArticleAssertHandler::assert_article_aggregation(
            patched_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                enabled: false,

                archived: false,
                name: String::from(creation_body.name.clone()),
                content: String::from(creation_body.content.clone()),
                language: String::from(creation_body.language.clone()),
            }),
            ArticleAssertOptions { is_updated: true },
        );

        let patched_article = ArticleRequestHandler::patch_article(
            &setup,
            created_article.id,
            &ArticlePatchBody { enabled: true },
        );

        ArticleAssertHandler::assert_article_aggregation(
            patched_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                enabled: true,

                archived: false,
                name: String::from(creation_body.name.clone()),
                content: String::from(creation_body.content.clone()),
                language: String::from(creation_body.language.clone()),
            }),
            ArticleAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn patch_nonexisting_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let response =
            ArticleRequest::patch_article(&setup, 0, &ArticlePatchBody { enabled: false });

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());
    }
}

mod delete_restore_article_tests {
    use super::*;

    #[test]
    fn delete_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test delete article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let deleted_article = ArticleRequestHandler::delete_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            deleted_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                archived: true,

                enabled: true,
                name: String::from(creation_body.name),
                content: String::from(creation_body.content),
                language: String::from(creation_body.language),
            }),
            ArticleAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn delete_nonexisting_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let response = ArticleRequest::delete_article(&setup, 0);

        assert_eq!(response.status(), Status::NotFound);
        let error_message = response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());
    }

    #[test]
    fn restore_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test restore article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let deleted_article = ArticleRequestHandler::delete_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            deleted_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                archived: true,

                enabled: true,
                name: String::from(creation_body.name.clone()),
                content: String::from(creation_body.content.clone()),
                language: String::from(creation_body.language.clone()),
            }),
            ArticleAssertOptions { is_updated: true },
        );

        let restored_article = ArticleRequestHandler::restore_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            restored_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                archived: false,

                enabled: true,
                name: String::from(creation_body.name),
                content: String::from(creation_body.content),
                language: String::from(creation_body.language),
            }),
            ArticleAssertOptions { is_updated: true },
        );
    }

    #[test]
    fn restore_nonexisting_article() {
        let setup = TestSetup::new(SetupOptions { is_lock: true });

        let creation_body = ArticleCreateRelationsDto {
            name: String::from("test restore article"),
            content: String::from("test content"),
            language: String::from("ua"),
        };

        let created_article = ArticleRequestHandler::create_article(&setup, &creation_body);

        let deleted_article = ArticleRequestHandler::delete_article(&setup, created_article.id);

        ArticleAssertHandler::assert_article_aggregation(
            deleted_article,
            ArticleMockHandler::get_article_aggregation(ArticleMockOptions {
                archived: true,

                enabled: true,
                name: String::from(creation_body.name.clone()),
                content: String::from(creation_body.content.clone()),
                language: String::from(creation_body.language.clone()),
            }),
            ArticleAssertOptions { is_updated: true },
        );

        let nonexisting_article_id = 0;
        let restored_article_response =
            ArticleRequest::restore_article(&setup, nonexisting_article_id);

        assert_eq!(restored_article_response.status(), Status::NotFound);
        let error_message = restored_article_response.into_string().unwrap();

        assert_eq!(error_message, FmtError::NotFound("article").fmt());
    }
}
