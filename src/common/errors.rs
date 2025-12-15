use std::{path::PathBuf, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("PathBuf conversion error: {0}")]
    PathBufConversionError(PathBuf),

    #[error("Invalid sampling factor: {0}")]
    InvalidSamplingError(f32),

    #[error("File does not exist: {0}")]
    FileDoesNotExistError(PathBuf),

    #[error("Invalid file extension: {0}")]
    InvalidFileExtension(PathBuf),

    #[error("Failed to open file")]
    FailedToOpenFileError(String),

    #[error("Failed to parse file")]
    FailedToParseFileError(String),

    #[error("Serialization error")]
    SerializationError(String),

    #[error("Regex parsing error")]
    RegexParsingError(String),

    #[error("Regex capture error")]
    RegexCaptureError(String),

    #[error("Utf8 encoding error")]
    Utf8EncodingError(String),

    #[error("Invalid argument")]
    InvalidArgumentError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FailedToOpenFileError(err.to_string())
    }
}

impl From<needletail::errors::ParseError> for AppError {
    fn from(err: needletail::errors::ParseError) -> Self {
        AppError::FailedToParseFileError(err.to_string())
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::Utf8EncodingError(err.to_string())
    }
}
