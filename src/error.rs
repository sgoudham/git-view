#[derive(Debug, PartialEq)]
pub enum ErrorType {
    CommandFailed,
    CommandError,
    MissingGitRepository,
    MissingGitRemote,
    MissingDefaultBranch,
    InvalidGitUrl,
    InvalidUtf8,
    IOError,
}

#[derive(Debug, PartialEq)]
pub struct AppError {
    pub error_type: ErrorType,
    pub error_str: String,
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::new(ErrorType::IOError, error.to_string())
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(error: std::str::Utf8Error) -> Self {
        AppError::new(ErrorType::InvalidUtf8, error.to_string())
    }
}

impl AppError {
    pub fn new(error_type: ErrorType, error_str: String) -> Self {
        Self {
            error_type,
            error_str,
        }
    }
}
