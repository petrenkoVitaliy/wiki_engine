use rocket::{http::Status, response::status};

use super::formatted_error::FmtError;

#[derive(Debug)]
pub struct ErrorWrapper {
    pub status: Status,
    pub message: String,
    pub extra_message: Option<String>,
}

impl ErrorWrapper {
    pub fn new(fmt_error: &FmtError, extra_message: Option<String>) -> Self {
        let status = match fmt_error {
            FmtError::NotFound(_) => Status::NotFound,
            FmtError::AlreadyExists(_) => Status::BadRequest,
            FmtError::FailedToProcess(_) => Status::NotAcceptable,
            FmtError::EmptyValue(_) => Status::NotAcceptable,
            FmtError::Unauthorized(_) => Status::Unauthorized,
            FmtError::PermissionDenied(_) => Status::Forbidden,
            _ => Status::ImATeapot,
        };

        return ErrorWrapper {
            status,
            message: fmt_error.fmt(),
            extra_message,
        };
    }

    pub fn custom(&self) -> status::Custom<String> {
        status::Custom(self.status, self.message.clone())
    }
}
