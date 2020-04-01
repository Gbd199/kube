use thiserror::Error;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

#[derive(Error, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[error("{message}: {reason}")]
pub struct ErrorResponse {
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub reason: String,
    pub code: u16,
}

#[derive(Error, Debug)]
pub enum Error {
    /// ApiError for when things fail
    ///
    /// This can be parsed into as an error handling fallback.
    /// Replacement data for reqwest::Response::error_for_status,
    /// which is often lacking in good permission errors.
    /// It's also used in `WatchEvent` from watch calls.
    ///
    /// It's quite common to get a `410 Gone` when the resourceVersion is too old.
    #[error("ApiError: {0} ({0:?})")]
    Api(ErrorResponse),

    // Request errors
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("HttpError: {0}")]
    HttpError(#[from] http::Error),

    /// Common error case when requesting parsing into own structs
    #[error("Error deserializing response")]
    SerdeError(#[from] serde_json::Error),

    #[error("Error building request")]
    RequestBuild,
    #[error("Error executing request")]
    RequestSend,
    #[error("Error parsing response")]
    RequestParse,
    #[error("Invalid API method {0}")]
    InvalidMethod(String),
    #[error("Request validation failed with {0}")]
    RequestValidation(String),

    /// Configuration error
    #[error("Error loading kubeconfig: {0}")]
    Kubeconfig(String),

    #[error("SslError: {0}")]
    SslError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod api;
pub use api::{Api, Resource};
pub mod client;
#[doc(inline)] pub use client::Client;
pub mod config;
#[doc(inline)] pub use config::Config;
mod oauth2;
pub mod runtime;