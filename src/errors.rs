use std::str::Utf8Error;

use bio_utils_rs::errors::BioError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid sampling factor: {0}")]
    InvalidSamplingError(f32),

    #[error("Regex parsing error")]
    RegexParsingError(String),

    #[error("Utf8 encoding error")]
    Utf8EncodingError(String),

    #[error("Invalid argument")]
    InvalidArgumentError(String),

    #[error(transparent)]
    BioError(#[from] BioError),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::BioError(BioError::IoError(err))
    }
}

impl From<needletail::errors::ParseError> for AppError {
    fn from(err: needletail::errors::ParseError) -> Self {
        AppError::BioError(BioError::NeedletailParseError(err))
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::Utf8EncodingError(err.to_string())
    }
}
