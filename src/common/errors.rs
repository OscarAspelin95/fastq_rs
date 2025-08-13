use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("FastqError")]
    FastqError,
}
