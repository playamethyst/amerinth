pub use api::misc;
pub use client::{Modrinth, UserAgent};

// todo: utils
// - mrpack downloader
// - curseforge -> modrinth

// todo: blocking apis

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
}
