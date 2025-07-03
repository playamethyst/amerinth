pub use auth::{ModrinthAuth, UserAgent};

// todo: endpoints
// - projects
// - versions
// - version-files
// - users
// - notifications
// - threads
// - teams
// - tags
// - misc

// todo: utils
// - mrpack downloader
// - curseforge -> modrinth

// todo: blocking apis

mod auth;

/// An error that can occur when using the Modrinth API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Auth(#[from] auth::AuthError),
}
