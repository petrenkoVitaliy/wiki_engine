use rocket::{http::Status, response::status};

use super::formatted_error::FmtError;

#[derive(Debug)]
pub struct ErrorWrapper {
    pub status: Status,
    pub message: String,
}

impl ErrorWrapper {
    pub fn new(fmt_error: &FmtError) -> Self {
        match fmt_error {
            FmtError::NotFound(_) => ErrorWrapper {
                status: Status::NotFound,
                message: fmt_error.fmt(),
            },
            FmtError::AlreadyExists(_) => ErrorWrapper {
                status: Status::BadRequest,
                message: fmt_error.fmt(),
            },
            FmtError::FailedToProcess(_) => ErrorWrapper {
                status: Status::NotAcceptable,
                message: fmt_error.fmt(),
            },
            _ => ErrorWrapper {
                status: Status::ImATeapot,
                message: String::from("I'm a teapot"),
            },
        }
    }

    pub fn custom(&self) -> status::Custom<String> {
        status::Custom(self.status, self.message.clone())
    }
}
