pub use api::misc;
pub use client::{Modrinth, UserAgent};

mod api;
mod client;

/// An error that can occur when using the Modrinth API.
#[derive(Debug, thiserror::Error)]
pub enum ModrinthError {
    #[error("Failed to build HTTP client: {0}")]
    HttpBuild(#[from] reqwest::Error),
    #[error("Invalid date provided for expiration: {0}")]
    InvalidDate(#[from] time::error::ComponentRange),
    #[error("Token is invalid or expired")]
    InvalidToken,
    #[error("Failed to parse response: {0}")]
    ClientError(#[from] rustify::errors::ClientError),
}

pub(crate) mod prelude {
    pub(crate) use crate::ModrinthError;
    pub(crate) use crate::client::{AuthState, Modrinth};
    pub(crate) use rustify::Endpoint;
    pub(crate) use rustify_derive::Endpoint;
    pub(crate) use serde::Deserialize;
}
