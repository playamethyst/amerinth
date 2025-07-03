pub use api::{misc, users};
pub use client::{Modrinth, UserAgent};

mod api;
mod client;

/// An error that can occur when using the Modrinth API.
#[derive(Debug, thiserror::Error)]
pub enum ModrinthError {
    #[error("Invalid date provided for expiration: {0}")]
    InvalidDate(#[from] time::error::ComponentRange),
    #[error("Token is invalid or expired")]
    InvalidToken,
    #[error("Failed to parse response: {0}")]
    ClientError(#[from] rustify::errors::ClientError),
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use crate::client::{AuthMiddleware, AuthState, Authenticated, Modrinth};
    pub(crate) use rustify::Endpoint;
    pub(crate) use rustify::errors::ClientError;
    pub(crate) use rustify_derive::Endpoint;
    pub(crate) use serde::Deserialize;
    pub(crate) use thiserror::Error;
}
