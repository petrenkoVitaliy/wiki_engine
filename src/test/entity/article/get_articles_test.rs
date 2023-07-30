use super::setup::{SetupOptions, TestSetup, TestUser};

use super::test_handler::{
    assert_handler::article::{ArticleAssertHandler, ArticleAssertOptions},
    mock_handler::article::{ArticleMockHandler, ArticleMockOptions},
    request_handler::article::ArticleRequestHandler,
};

use super::repository::entity::article::ArticleType;
use super::schema::article::{ArticleCreateRelationsBody, ArticlePatchBody};

#[tokio::test]
async fn get_articles() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: String::from("test get articles"),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token).await;

    let received_articles = ArticleRequestHandler::get_articles(&setup).await;

    let expected_article = received_articles
        .into_iter()
        .find(|article| article.id == created_article.id)
        .expect("Article not found");

    assert_eq!(expected_article.languages[0].name, creation_body.name);

    ArticleAssertHandler::assert_article_aggregation(
        expected_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: false },
    );
}

#[tokio::test]
async fn get_disabled_articles() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: String::from("test get disabled articles"),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody { enabled: false },
        admin_token.clone(),
    )
    .await;

    let received_articles = ArticleRequestHandler::get_articles(&setup).await;

    let expected_article = received_articles
        .into_iter()
        .find(|article| article.id == created_article.id);

    assert_eq!(expected_article.is_none(), true);

    ArticleRequestHandler::patch_article(
        &setup,
        created_article.id,
        &ArticlePatchBody { enabled: true },
        admin_token,
    )
    .await;

    let received_articles = ArticleRequestHandler::get_articles(&setup).await;

    let expected_article = received_articles
        .into_iter()
        .find(|article| article.id == created_article.id)
        .expect("Article not found");

    assert_eq!(expected_article.languages[0].name, creation_body.name);

    ArticleAssertHandler::assert_article_aggregation(
        expected_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: true },
    );
}

#[tokio::test]
async fn get_deleted_articles() {
    let setup = TestSetup::new(SetupOptions { is_lock: true }).await;
    let admin_token = setup.user_handler.get_token(TestUser::Admin1).unwrap();

    let creation_body = ArticleCreateRelationsBody {
        name: String::from("test get disabled articles"),
        content: String::from("test content"),
        language: String::from("ua"),
        article_type: ArticleType::Public,
    };

    let created_article =
        ArticleRequestHandler::create_article(&setup, &creation_body, admin_token.clone()).await;

    ArticleRequestHandler::delete_article(&setup, created_article.id, admin_token.clone()).await;

    let received_articles = ArticleRequestHandler::get_articles(&setup).await;

    let expected_article = received_articles
        .into_iter()
        .find(|article| article.id == created_article.id);

    assert_eq!(expected_article.is_none(), true);

    ArticleRequestHandler::restore_article(&setup, created_article.id, admin_token).await;

    let received_articles = ArticleRequestHandler::get_articles(&setup).await;

    let expected_article = received_articles
        .into_iter()
        .find(|article| article.id == created_article.id)
        .expect("Article not found");

    assert_eq!(expected_article.languages[0].name, creation_body.name);

    ArticleAssertHandler::assert_article_aggregation(
        expected_article,
        ArticleMockHandler::get_article_aggregation(ArticleMockOptions::from_creation_body(
            creation_body,
        )),
        ArticleAssertOptions { is_updated: true },
    );
}
