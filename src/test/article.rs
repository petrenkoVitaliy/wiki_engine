use rocket::{http::Status, uri};

use super::launch::get_client;

use super::aggregation::article::ArticleAggregation;
use super::schema::article::ArticleCreateRelationsDto;

use super::router::article::*;

#[test]
fn create_article() {
    let client = get_client();

    let creation_body = ArticleCreateRelationsDto {
        name: String::from("test name 1"),
        content: String::from("test content"),
        language: String::from("ua"),
    };

    let response = client
        .post(uri!("/articles", create_article))
        .json::<ArticleCreateRelationsDto>(&creation_body)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_json::<ArticleAggregation>().unwrap();

    print!("{:?}", response_body);

    assert_eq!(response_body.archived, false);
    assert_eq!(response_body.enabled, true);
    // assert_eq!(response_body.updated_at, None); TODO

    let article_language = response_body
        .languages
        .get(0)
        .expect("article_language should exist");
    assert_eq!(response_body.languages.len(), 1);
    assert_eq!(article_language.name, creation_body.name);
    assert_eq!(article_language.enabled, true);
    assert_eq!(article_language.archived, false);

    assert_eq!(article_language.language.code, "ua");

    let article_version = article_language
        .versions
        .get(0)
        .expect("article_version should exist");
    assert_eq!(article_language.versions.len(), 1);
    assert_eq!(article_version.version, 1);
    assert_eq!(article_version.enabled, true);

    assert_eq!(article_version.content.content, creation_body.content);
}
