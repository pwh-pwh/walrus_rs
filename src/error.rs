use reqwest::StatusCode;
use thiserror::Error;

/// The `WalrusError` enum represents all possible errors that can occur within the `walrus_rs` library.
///
/// It encapsulates HTTP request errors, URL parsing errors, API-specific errors, response parsing errors, and other general errors.
#[derive(Error, Debug)]
pub enum WalrusError {
    /// An HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    /// An invalid URL was provided.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    /// An error returned by the Walrus API. Contains the HTTP status code and error message.
    #[error("API error: {0} - {1}")]
    ApiError(StatusCode, String),
    /// Failed to parse the response.
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    /// An invalid parameter was provided.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    /// An unknown error occurred.
    #[error("Unknown error: {0}")]
    Unknown(String),
    /// A general or other error occurred.
    #[error("Other error: {0}")]
    Other(String),
}
