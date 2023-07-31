use rocket::local::asynchronous::Client;
use std::collections::HashMap;

use super::dtm::auth::request_body::{UserLoginBody, UserSignupBody};
use super::dtm_common::UserRoleId;
use super::test_handler::request_handler::auth::AuthRequestHandler;

#[derive(std::cmp::Eq, PartialEq, Hash, Copy, Clone)]
pub enum TestUser {
    Admin1,
    Common1,
}

#[derive(Debug)]
pub struct TestUserCredentials {
    pub name: String,
    pub role_id: UserRoleId,
    pub login_options: UserLoginBody,
    pub auth_token: Option<String>,
}

pub struct TestUsersHandler {
    pub credentials: HashMap<TestUser, TestUserCredentials>,
}

impl TestUsersHandler {
    pub fn get_token(&self, user: TestUser) -> Option<String> {
        match self.credentials.get(&user).unwrap().auth_token.as_ref() {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    pub fn new() -> Self {
        Self::get_users_credentials()
    }

    pub async fn create_users(&mut self, client: &Client) {
        for (_, user_credentials) in self.credentials.iter_mut() {
            let token_response = AuthRequestHandler::signup_with_role(
                client,
                &UserSignupBody {
                    name: user_credentials.name.clone(),
                    email: user_credentials.login_options.email.clone(),
                    password: user_credentials.login_options.password.clone(),
                },
                user_credentials.role_id.clone() as i32,
            )
            .await;

            user_credentials.auth_token = Some(token_response.token);
        }
    }

    fn get_users_credentials() -> Self {
        Self {
            credentials: HashMap::from([
                (
                    TestUser::Admin1,
                    TestUserCredentials {
                        auth_token: None,
                        name: String::from("admin"),
                        role_id: UserRoleId::Admin,
                        login_options: UserLoginBody {
                            email: String::from("admin@mail.com"),
                            password: String::from("password"),
                        },
                    },
                ),
                (
                    TestUser::Common1,
                    TestUserCredentials {
                        auth_token: None,
                        name: String::from("user"),
                        role_id: UserRoleId::Common,
                        login_options: UserLoginBody {
                            email: String::from("user@mail.com"),
                            password: String::from("password"),
                        },
                    },
                ),
            ]),
        }
    }
}
