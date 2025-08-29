use std::path::PathBuf;
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
}
