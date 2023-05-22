use dotenv::dotenv;
use rocket::launch;

mod error;
mod option_config;
mod repository;
mod router;
mod schema;
mod service;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(repository::connection::PgConnection::fairing())
        .mount("/articles", router::article::routes())
}
