// todo:
// - GET /updates/{id|slug}/forge_updates.json

use crate::{
    Modrinth, ModrinthError,
    client::{AuthState, HttpClient},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Statistics {
    /// Number of projects on Modrinth
    projects: usize,
    /// Number of versions on Modrinth
    versions: usize,
    /// Number of version files on Modrinth
    files: usize,
    /// Number of authors (users with projects) on Modrinth
    authors: usize,
}

/// ### GET `/statistics`
pub async fn statistics<State: AuthState>(
    auth: &Modrinth<State>,
) -> Result<Statistics, ModrinthError>
where
    State: AuthState,
    Modrinth<State>: HttpClient,
{
    auth.get("/statistics").await
}
