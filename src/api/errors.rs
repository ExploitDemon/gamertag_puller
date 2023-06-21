// src/api/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GamertagError {
    #[error("Request Error: {0}")]
    RequestError(String),

    #[error("API Call Failed: {status}, {message}")]
    ApiCallFailed {
        status: u16,
        message: String,
    },
}
