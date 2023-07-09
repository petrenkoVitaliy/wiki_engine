use dotenv::dotenv;
use parking_lot::MutexGuard;
use rocket::{local::blocking::Client, Build, Rocket};

use super::repository;
use super::router::article;
use super::router::article_language;
use super::router::article_version;
use super::router::catchers;

pub struct SetupOptions {
    pub is_lock: bool,
}

pub struct TestSetup {
    pub client: Client,
    pub lock: Option<MutexGuard<'static, ()>>,
}

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

impl TestSetup {
    pub fn new(options: SetupOptions) -> Self {
        let lock = match options.is_lock {
            true => Some(DB_LOCK.lock()),
            _ => None,
        };

        return Self {
            client: Self::get_client(),
            lock,
        };
    }

    fn get_rocket() -> Rocket<Build> {
        dotenv().ok();

        rocket::build()
            .attach(repository::connection::PgConnection::fairing())
            .mount("/articles", article::routes())
            .mount("/articles", article_language::routes())
            .mount("/articles", article_version::routes())
            .register("/", catchers::catchers())
    }

    fn get_client() -> Client {
        Client::tracked(Self::get_rocket()).expect("valid rocket instance")
    }
}
