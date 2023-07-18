use super::error_wrapper::ErrorWrapper;

pub enum FmtError<'input> {
    FailedToProcess(&'input str),
    NotFound(&'input str),
    FailedToFetch(&'input str),
    FailedToInsert(&'input str),
    FailedToUpdate(&'input str),
    AlreadyExists(&'input str),
}

impl<'input> FmtError<'input> {
    pub fn fmt(&self) -> String {
        match self {
            FmtError::NotFound(s) => format!("Entity not found: {}", s),
            FmtError::FailedToFetch(s) => format!("Failed to fetch: {}", s),
            FmtError::FailedToInsert(s) => format!("Failed to insert: {}", s),
            FmtError::FailedToUpdate(s) => format!("Failed to update: {}", s),
            FmtError::AlreadyExists(s) => format!("Entity already exists: {}", s),
            FmtError::FailedToProcess(s) => format!("Failed to process entity: {}", s),
        }
    }

    pub fn error<T>(&self) -> Result<T, ErrorWrapper> {
        Err(ErrorWrapper::new(self))
    }

    pub fn error_wrapper(&self) -> ErrorWrapper {
        ErrorWrapper::new(self)
    }
}
