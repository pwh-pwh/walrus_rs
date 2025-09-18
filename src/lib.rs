//! `walrus_rs` is a Rust client library for interacting with the Walrus API.
//! It provides both asynchronous and blocking clients for users to choose from based on their needs.
//!
//! Key features include:
//! - Asynchronous and blocking communication with the Walrus API.
//! - Serialization/deserialization of requests and responses.
//! - Definition of API-related data models.
//! - Unified error handling mechanism.
//!
//! Module overview:
//! - [`client`]: Provides the asynchronous Walrus client [`WalrusClient`].
//! - [`blocking_client`]: Provides the blocking Walrus client [`BlockingWalrusClient`].
//! - [`models`]: Defines the data structures used by the Walrus API.
//! - [`error`]: Defines the library's error types [`WalrusError`].
//!
//! Examples:
//!
//! Asynchronous client usage example:
//! ```no_run
//! use walrus_rs::WalrusClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), walrus_rs::WalrusError> {
//!     let client = WalrusClient::new("your_api_key".to_string());
//!     // Perform API calls
//!     // let response = client.some_api_call().await?;
//!     // println!("{:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! Blocking client usage example:
//! ```no_run
//! use walrus_rs::BlockingWalrusClient;
//!
//! fn main() -> Result<(), walrus_rs::WalrusError> {
//!     let client = BlockingWalrusClient::new("your_api_key".to_string());
//!     // Perform API calls
//!     // let response = client.some_api_call()?;
//!     // println!("{:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! [`client`]: crate::client
//! [`blocking_client`]: crate::blocking_client
//! [`models`]: crate::models
//! [`error`]: crate::error
//! [`WalrusClient`]: crate::client::WalrusClient
//! [`BlockingWalrusClient`]: crate::blocking_client::BlockingWalrusClient
//! [`WalrusError`]: crate::error::WalrusError

pub mod client;
pub mod error;
pub mod models;

pub mod blocking_client;

pub use blocking_client::BlockingWalrusClient;
pub use client::WalrusClient;
pub use error::WalrusError;
