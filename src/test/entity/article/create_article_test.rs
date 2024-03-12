use rocket::http::Status;

use super::error::FmtError;
use super::setup::{SetupOptions, TestSetup, TestUser};

use super::test_handler::{
    assert_handler::article::{ArticleAssertHandler, ArticleAssertOptions},
    mock_handler::article::{ArticleMockHandler, ArticleMockOptions},
    request_handler::article::{ArticleRequest, ArticleRequestHandler},
};

use super::dtm::article::request_body::ArticleCreateRelationsBody;
use super::repository::entity::article::ArticleType;

#[tokio::test]
async fn create_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from("test create article content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let response_body =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token).await;

    ArticleAssertHandler::assert_article_aggregation(
        response_body,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn create_article_with_wrong_language() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_wrong_article", setup.test_id),
        content: String::from("test content"),
        language: String::from("incorrect"),
        article_type: ArticleType::Public,
    };

    let response = ArticleRequest::create_article(&setup, &creation_body, admin_token).await;

    assert_eq!(response.status(), Status::NotFound);
    let error_message = response.into_string().await.unwrap();

    assert_eq!(error_message, FmtError::NotFound("language").fmt());
}

#[tokio::test]
async fn create_large_article() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

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

    let creation_body = ArticleCreateRelationsBody {
        name: format!("{}_article", setup.test_id),
        content: String::from(content),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let response_body =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token).await;

    ArticleAssertHandler::assert_article_aggregation(
        response_body,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: false },
    );
}
