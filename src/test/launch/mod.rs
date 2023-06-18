use super::repository;
use super::router::article;
use dotenv::dotenv;

use rocket::{local::blocking::Client, Build, Rocket};

fn test_rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .attach(repository::connection::PgConnection::fairing())
        .mount("/articles", article::routes())
}

pub fn get_client() -> Client {
    Client::tracked(test_rocket()).expect("valid rocket instance")
}
