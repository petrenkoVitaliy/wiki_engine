use rocket::{http::Status, uri};

use super::launch::get_client;

use super::aggregation::article::ArticleAggregation;

#[test]
fn get_articles() {
    let client = get_client();

    let response = client.get(uri!("/articles")).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_body: Vec<ArticleAggregation> = response.into_json().unwrap();

    // cargo test -- --nocapture
    print!("{:#?}", response_body);

    assert_eq!(response_body.len(), 1);
}
