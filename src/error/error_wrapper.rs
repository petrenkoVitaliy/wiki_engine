use diesel::result;
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

            FmtError::DatabaseError(_) => Status::InternalServerError,
            _ => Status::ImATeapot,
        };

        return ErrorWrapper {
            status,
            message: fmt_error.fmt(),
            extra_message,
        };
    }

    pub fn from_duplicated_key(error: result::Error, alternative_error: Self) -> Self {
        let error_srt = error.to_string();
        let duplicate_key_prefix = "duplicate key value violates unique constraint ";

        if !error_srt.contains(duplicate_key_prefix) {
            return alternative_error;
        }

        let mut split = error_srt.splitn(2, duplicate_key_prefix);

        split.next();
        let duplicated_key = split.next();

        match duplicated_key {
            Some(duplicated_key) => match duplicated_key {
                "\"user_account_email_key\"" => Self::new(&FmtError::AlreadyExists("email"), None),
                "\"user_account_name_key\"" => Self::new(&FmtError::AlreadyExists("name"), None),
                _ => Self::new(&FmtError::DatabaseError(error.to_string().as_str()), None),
            },
            None => Self::new(&FmtError::DatabaseError(error.to_string().as_str()), None),
        }
    }

    pub fn custom(&self) -> status::Custom<String> {
        status::Custom(self.status, self.message.clone())
    }
}

impl From<result::Error> for ErrorWrapper {
    fn from(error: result::Error) -> ErrorWrapper {
        return Self::new(&FmtError::DatabaseError(error.to_string().as_str()), None);
    }
}
