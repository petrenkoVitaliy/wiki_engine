use rocket::launch;

use dotenv::dotenv;
use rocket_okapi::{
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

mod diff_handler;
mod error;
mod mapper;
mod option_config;
mod repository;
mod router;
mod schema;
mod service;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        urls: vec![
            UrlObject {
                name: "article version".to_string(),
                url: "/articles/article_version.json".to_string(),
            },
            UrlObject {
                name: "article language".to_string(),
                url: "/articles/article_language.json".to_string(),
            },
            UrlObject {
                name: "article".to_string(),
                url: "/articles/article.json".to_string(),
            },
        ],

        display_operation_id: true,
        deep_linking: true,
        default_model_expand_depth: 3,
        show_extensions: true,
        show_common_extensions: true,
        max_displayed_tags: 3,

        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(repository::connection::PgConnection::fairing())
        .mount("/articles", router::article::routes())
        .mount("/articles", router::article_language::routes())
        .mount("/articles", router::article_version::routes())
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
