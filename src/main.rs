use rocket::launch;

mod error;
mod repository;
mod router;
mod schema;
mod service;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(repository::connection::PgConnection::fairing())
        .mount("/articles", router::article::routes())
}
