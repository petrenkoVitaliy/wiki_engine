use super::error_wrapper::ErrorWrapper;

pub enum FmtError<'input> {
    FailedToProcess(&'input str),
    EmptyValue(&'input str),
    NotFound(&'input str),
    AlreadyExists(&'input str),

    FailedToFetch(&'input str),
    FailedToInsert(&'input str),
    FailedToUpdate(&'input str),

    Unauthorized(&'input str),
    PermissionDenied(&'input str),

    DatabaseError(&'input str),

    FailedToSendRequest(&'input str),
}

impl<'input> FmtError<'input> {
    pub fn fmt(&self) -> String {
        match self {
            FmtError::FailedToProcess(s) => format!("Failed to process entity: {}", s),
            FmtError::EmptyValue(s) => format!("Empty: {}", s),
            FmtError::NotFound(s) => format!("Entity not found: {}", s),
            FmtError::AlreadyExists(s) => format!("Entity already exists: {}", s),

            FmtError::FailedToFetch(s) => format!("Failed to fetch: {}", s),
            FmtError::FailedToInsert(s) => format!("Failed to insert: {}", s),
            FmtError::FailedToUpdate(s) => format!("Failed to update: {}", s),

            FmtError::Unauthorized(s) => format!("Not authorized: {}", s),
            FmtError::PermissionDenied(s) => format!("Permission denied: {}", s),

            FmtError::DatabaseError(s) => format!("Database error: {}", s),

            FmtError::FailedToSendRequest(s) => format!("Request error: {}", s),
        }
    }

    pub fn error<T>(&self) -> Result<T, ErrorWrapper> {
        Err(ErrorWrapper::new(self, None))
    }

    pub fn error_wrapper(&self) -> ErrorWrapper {
        ErrorWrapper::new(self, None)
    }

    pub fn error_wrapper_enriched(&self, extra_message: String) -> ErrorWrapper {
        ErrorWrapper::new(self, Some(extra_message))
    }
}
