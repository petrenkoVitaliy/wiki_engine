use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket::{http::Status, uri};
use serde::Serialize;

use super::router::auth::*;

use super::dtm_common::TokenDto;

use super::setup::TestSetup;

pub struct AuthRequestHandler;
impl AuthRequestHandler {
    pub async fn signup_with_role<T>(client: &Client, creation_body: &T, role_id: i32) -> TokenDto
    where
        T: Serialize,
    {
        let response = AuthRequest::signup_with_role(client, creation_body, role_id).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<TokenDto>().await.unwrap()
    }

    pub async fn _login<T>(setup: &TestSetup, login_body: &T) -> TokenDto
    where
        T: Serialize,
    {
        let response = AuthRequest::_login(setup, login_body).await;

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<TokenDto>().await.unwrap()
    }
}

struct AuthRequest;
impl AuthRequest {
    pub async fn signup_with_role<'s, T>(
        client: &'s Client,
        creation_body: &T,
        role_id: i32,
    ) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        client
            .post(uri!("/auth", signup_with_role(role_id)))
            .json::<T>(creation_body)
            .dispatch()
            .await
    }

    pub async fn _login<'s, T>(setup: &'s TestSetup, login_body: &T) -> LocalResponse<'s>
    where
        T: Serialize,
    {
        setup
            .client
            .post(uri!("/auth", login))
            .json::<T>(login_body)
            .dispatch()
            .await
    }
}
