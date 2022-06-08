#[derive(Debug, PartialEq)]
pub enum AppError {
    CommandFailedToExecute(String),
    CommandError(String),
    MissingGitRepository(String),
    MissingGitRemote(String),
    InvalidGitUrl(String),
    InvalidUtf8(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::CommandFailedToExecute(error.to_string())
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(error: std::str::Utf8Error) -> Self {
        AppError::InvalidUtf8(error.to_string())
    }
}

impl AppError {
    pub fn print(&self) -> &String {
        match self {
            AppError::CommandFailedToExecute(str)
            | AppError::MissingGitRepository(str)
            | AppError::MissingGitRemote(str)
            | AppError::CommandError(str)
            | AppError::InvalidGitUrl(str)
            | AppError::InvalidUtf8(str) => str,
        }
    }
}
