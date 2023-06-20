use rocket::{http::Status, uri};

mod validator;
use validator::ArticleResponseValidator;

mod expected_mock;
use expected_mock::ArticleExpectedMock;

use super::router::article::*;
use super::setup::{SetupOptions, TestSetup};

use super::aggregation::article::ArticleAggregation;
use super::aggregation::article_language::ArticleLanguageAggregation;
use super::aggregation::article_version::ArticleVersionAggregation;
use super::aggregation::language::LanguageAggregation;
use super::aggregation::version_content::VersionContentAggregation;

use super::schema::article::ArticleCreateRelationsDto;

#[test]
fn create_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true });

    let creation_body = ArticleCreateRelationsDto {
        name: String::from("test name 1"),
        content: String::from("test content"),
        language: String::from("ua"),
    };

    let response = setup
        .client
        .post(uri!("/articles", create_article))
        .json::<ArticleCreateRelationsDto>(&creation_body)
        .dispatch();

    print!("{:?}", response.body());
    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_json::<ArticleAggregation>().unwrap();

    ArticleResponseValidator::validate_article_aggregation(
        response_body,
        ArticleExpectedMock::get_article_aggregation(creation_body),
    );
}
