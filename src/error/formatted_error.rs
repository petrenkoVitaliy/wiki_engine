pub enum FmtError<'input> {
    NotFound(&'input str),
    FailedToFetch(&'input str),
    FailedToInsert(&'input str),
}

impl<'input> FmtError<'input> {
    pub fn fmt(&self) -> String {
        match self {
            FmtError::FailedToFetch(s) => format!("Failed to fetch: {}", s),
            FmtError::FailedToInsert(s) => format!("Failed to insert: {}", s),
            FmtError::NotFound(s) => format!("Entity not found: {}", s),
        }
    }
}
