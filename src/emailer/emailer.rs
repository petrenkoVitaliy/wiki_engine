use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use rocket::serde;

use super::error::{ErrorWrapper, FmtError};

const BREVO_SECRET: &str = "BREVO_SECRET";
const CLIENT_URL: &str = "CLIENT_URL";

pub struct Emailer;

impl Emailer {
    fn get_headers() -> HeaderMap {
        let secret = env::var(BREVO_SECRET).expect(&FmtError::EmptyValue(BREVO_SECRET).fmt());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert(
            "api-key",
            HeaderValue::from_str(&secret).expect(&FmtError::FailedToProcess(BREVO_SECRET).fmt()),
        );

        return headers;
    }

    fn get_confirmation_body(url: String) -> serde::json::Value {
        return serde::json::json!({
            "to": [
                {
                    "email": "vitaliy.ptt@gmail.com",
                    "name": "User"
                }
            ],
            "templateId": 1,
            "params": {
                "url": url
            },
            "headers": {
                "X-Mailin-custom": "custom_header_1:custom_value_1|custom_header_2:custom_value_2|custom_header_3:custom_value_3",
                "charset": "iso-8859-1"
            }
        });
    }

    fn get_confirmation_url(otp: &String, email: &String, redirect_to: &Option<String>) -> String {
        let client_urn = env::var(CLIENT_URL).expect(&FmtError::EmptyValue(CLIENT_URL).fmt());

        let from_query = match redirect_to {
            Some(from) => format!("&from={from}"),
            None => String::from(""),
        };

        format!("{client_urn}/api/confirm?key={otp}&email={email}{from_query}")
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

        let json = Self::get_confirmation_body(url);

        match client
            .request(reqwest::Method::POST, "https://api.brevo.com/v3/smtp/email")
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
