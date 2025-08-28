use thiserror::Error;
use reqwest::StatusCode;

#[derive(Error, Debug)]
pub enum WalrusError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("API error: {0} - {1}")]
    ApiError(StatusCode, String),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}