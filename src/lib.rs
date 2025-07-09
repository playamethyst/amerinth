use crate::helpers::use_all;

use_all!(pub api);

mod client;
pub use client::{Modrinth, UserAgent};

mod helpers;

/// An error that can occur when using the Modrinth API.
#[derive(Debug, thiserror::Error)]
pub enum ModrinthError {
    /// Resource not found.
    #[error(r#"Resource not found: {resource} "{id}""#)]
    NotFound { resource: &'static str, id: String },

    /// Client is unauthorized.
    #[error(r#"Unauthorized"#)]
    Unauthorized,

    /// Invalid expiration date.
    #[error(r#"Invalid expiration date: {0}/{1}/{2}"#)]
    Expiration(u8, u8, i32),

    #[error("Client error: {0}")]
    Client(#[from] rustify::errors::ClientError),

    #[error(transparent)]
    Infalliable(#[from] std::convert::Infallible),
    #[error(transparent)]
    Parse(#[from] strum::ParseError),
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use crate::client::{AuthState, Authenticated, Modrinth};
    pub(crate) use crate::{ModrinthError, helpers::*};
    pub(crate) use chrono::{DateTime, Utc};
    pub(crate) use rustify::Endpoint;
    pub(crate) use rustify::errors::ClientError;
    pub(crate) use serde::Deserialize;
    pub(crate) use strum::EnumString;
}
