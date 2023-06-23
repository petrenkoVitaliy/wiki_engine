use super::setup::{SetupOptions, TestSetup};
use super::test_handler::{
    assert_handler::article_language::ArticleLanguageAssertHandler,
    mock_handler::article_language::{ArticleLanguageMockHandler, ArticleLanguageMockOptions},
    request_handler::{
        article::ArticleRequestHandler, article_language::ArticleLanguageRequestHandler,
    },
};

use super::schema::{
    article::ArticleCreateRelationsDto, article_language::ArticleLanguageCreateBody,
};

#[test]
fn create_article_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let article = ArticleRequestHandler::create_article_handler(
        &setup,
        &ArticleCreateRelationsDto {
            name: String::from("test create article language"),
            content: String::from("test content"),
            language: String::from("ua"),
        },
    );

    let language = String::from("en");
    let creation_body = ArticleLanguageCreateBody {
        name: String::from("test create article"),
        content: String::from("test create article content"),
    };

    let response_body = ArticleLanguageRequestHandler::create_article_language_handler(
        &setup,
        &creation_body,
        article.id,
        language.clone(),
    );

    ArticleLanguageAssertHandler::assert_article_languages_aggregation(
        &response_body,
        &ArticleLanguageMockHandler::get_article_language_aggregation(
            ArticleLanguageMockOptions::from_creation_dto(creation_body, language),
        ),
    );
}
