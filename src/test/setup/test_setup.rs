use dotenv::dotenv;
use parking_lot::MutexGuard;
use rocket::{local::asynchronous::Client, Build, Rocket};
use std::sync::Once;

use super::repository;
use super::router::{article, article_language, article_version, auth, catchers};
use super::test_user_handler::TestUsersHandler;

pub struct SetupOptions {
    pub is_lock: bool,
}

static DB_LOCK: parking_lot::Mutex<()> = parking_lot::const_mutex(());

static mut TEST_USER_HANDLER: Option<TestUsersHandler> = None;
static TEST_USER_HANDLER_ONCE: Once = Once::new();

static mut CLIENT: Option<Client> = None;
static CLIENT_ONCE: Once = Once::new();

pub struct TestSetup {
    pub lock: Option<MutexGuard<'static, ()>>,
    pub client: &'static Client,
    pub user_handler: &'static TestUsersHandler,
}

impl TestSetup {
    pub async fn new(options: SetupOptions) -> Self {
        let lock = match options.is_lock {
            true => Some(DB_LOCK.lock()),
            _ => None,
        };

        let client = Self::get_client().await;
        let user_handler = Self::get_test_user_handler(&client).await;

        Self {
            client,
            lock,
            user_handler,
        }
    }

    async fn get_test_user_handler(client: &Client) -> &'static TestUsersHandler {
        if !TEST_USER_HANDLER_ONCE.is_completed() {
            TEST_USER_HANDLER_ONCE.call_once(|| {});

            let mut test_users_handler = TestUsersHandler::new();
            test_users_handler.create_users(&client).await;

            unsafe {
                TEST_USER_HANDLER = Some(test_users_handler);
            }
        }

        unsafe { TEST_USER_HANDLER.as_ref().unwrap() }
    }

    async fn get_client() -> &'static Client {
        if !CLIENT_ONCE.is_completed() {
            CLIENT_ONCE.call_once(|| {});

            let client = Client::tracked(Self::get_rocket())
                .await
                .expect("valid rocket instance");

            unsafe {
                CLIENT = Some(client);
            }
        }

        unsafe { CLIENT.as_ref().unwrap() }
    }

    fn get_rocket() -> Rocket<Build> {
        dotenv().ok();

        rocket::build()
            .attach(repository::PgConnection::fairing())
            .mount("/articles", article::routes())
            .mount("/articles", article_language::routes())
            .mount("/articles", article_version::routes())
            .mount("/auth", auth::test_routes())
            .register("/", catchers::catchers())
    }
}
