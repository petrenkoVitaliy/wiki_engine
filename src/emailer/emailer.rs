use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use rocket::serde;

use super::error::{ErrorWrapper, FmtError};

const BREVO_URL: &str = "https://api.brevo.com/v3/smtp/email";
const BREVO_SECRET_ENV: &str = "BREVO_SECRET";
const CLIENT_URL_ENV: &str = "CLIENT_URL";

pub struct Emailer;

impl Emailer {
    fn get_headers() -> HeaderMap {
        let secret =
            env::var(BREVO_SECRET_ENV).expect(&FmtError::EmptyValue(BREVO_SECRET_ENV).fmt());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert(
            "api-key",
            HeaderValue::from_str(&secret)
                .expect(&FmtError::FailedToProcess(BREVO_SECRET_ENV).fmt()),
        );

        return headers;
    }

    fn get_confirmation_body(url: String, email: &String) -> serde::json::Value {
        return serde::json::json!({
            "to": [
                {
                    "email": email,
                    "name": "User"
                }
            ],
            "templateId": 1,
            "params": {
                "url": url
            },
            "headers": {
                "charset": "iso-8859-1"
            }
        });
    }

    fn get_confirmation_url(otp: &String, email: &String, redirect_to: &Option<String>) -> String {
        let client_urn =
            env::var(CLIENT_URL_ENV).expect(&FmtError::EmptyValue(CLIENT_URL_ENV).fmt());

        let from_query = match redirect_to {
            Some(from) => format!("&from={from}"),
            None => String::from(""),
        };

        format!("{client_urn}/api/confirm?key={otp}&email={email}{from_query}")
    }

    fn get_reset_body(url: String, email: &String) -> serde::json::Value {
        return serde::json::json!({
            "to": [
                {
                    "email": email,
                    "name": "User"
                }
            ],
            "templateId": 2,
            "params": {
                "url": url
            },
            "headers": {
                "charset": "iso-8859-1"
            }
        });
    }

    fn get_reset_url(otp: &String, email: &String, redirect_to: &Option<String>) -> String {
        let client_urn =
            env::var(CLIENT_URL_ENV).expect(&FmtError::EmptyValue(CLIENT_URL_ENV).fmt());

        let from_query = match redirect_to {
            Some(from) => format!("&from={from}"),
            None => String::from(""),
        };

        format!("{client_urn}/reset/confirm?key={otp}&email={email}{from_query}")
    }

    pub async fn send_reset_email(
        otp: &String,
        email: &String,
        redirect_to: &Option<String>,
    ) -> Result<(), ErrorWrapper> {
        let client = reqwest::Client::builder()
            .build()
            .expect(&FmtError::FailedToProcess("emailer client").fmt());

        let url = Self::get_reset_url(otp, email, redirect_to);

        let json = Self::get_reset_body(url, email);

        match client
            .request(reqwest::Method::POST, BREVO_URL)
            .headers(Self::get_headers())
            .json(&json)
            .send()
            .await
        {
            Ok(_) => (),

            Err(err) => {
                eprintln!("{}", err);

                return Err(FmtError::FailedToSendRequest("emailer").error_wrapper());
            }
        }

        Ok(())
    }

    pub async fn send_confirmation_email(
        otp: &String,
        email: &String,
        redirect_to: &Option<String>,
    ) -> Result<(), ErrorWrapper> {
        let client = reqwest::Client::builder()
            .build()
            .expect(&FmtError::FailedToProcess("emailer client").fmt());

        let url = Self::get_confirmation_url(otp, email, redirect_to);

        let json = Self::get_confirmation_body(url, email);

        match client
            .request(reqwest::Method::POST, BREVO_URL)
            .headers(Self::get_headers())
            .json(&json)
            .send()
            .await
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);

                return Err(FmtError::FailedToSendRequest("emailer").error_wrapper());
            }
        }

        Ok(())
    }
}
