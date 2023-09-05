use dotenv::dotenv;
use rocket_okapi::{
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use rocket::{
    fairing::{Fairing, Info, Kind},
    options,
};
use rocket::{http::Header, routes};
use rocket::{Request, Response};

use super::{repository, router};

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
            UrlObject {
                name: "auth".to_string(),
                url: "/auth/auth.json".to_string(),
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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {}

pub fn launch() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();

    rocket::build()
        .attach(repository::PgConnection::fairing())
        .attach(CORS)
        .mount("/articles", router::article::routes())
        .mount("/articles", router::article_language::routes())
        .mount("/articles", router::article_version::routes())
        .mount("/languages", router::language::routes())
        .mount("/auth", router::auth::routes())
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .mount("/", routes![all_options,])
        .register("/", router::catchers::catchers())
}
