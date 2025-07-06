pub use api::{misc, tags, users};
pub use client::{Modrinth, UserAgent};

mod api;
mod client;
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
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use crate::client::{AuthState, Authenticated, Modrinth};
    pub(crate) use crate::{ModrinthError as Error, helpers::*};
    pub(crate) use rustify::Endpoint;
    pub(crate) use rustify::errors::ClientError;
    pub(crate) use rustify_derive::Endpoint;
    pub(crate) use serde::Deserialize;

    /// Execute an endpoint
    macro_rules! exec {
        ($endpoint:expr, $modrinth:expr) => {
            $endpoint
                .with_middleware(&$crate::client::AuthMiddleware($modrinth))
                .exec(&$modrinth.client)
                .await
        };
    }
    pub(crate) use exec;
}
