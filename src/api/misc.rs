// todo:
// - GET /updates/{id|slug}/forge_updates.json

use crate::{Modrinth, ModrinthError, client::AuthState};
use rustify::{errors::ClientError, Endpoint};
use rustify_derive::Endpoint;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "statistics", response = "Statistics")]
pub struct GetStatistics;

/// ### GET `/statistics`
pub async fn statistics<State: AuthState>(
    modrinth: &Modrinth<State>,
) -> Result<Statistics, ClientError>
where
    State: AuthState,
{
    GetStatistics.exec(&modrinth.client).await?.parse()
}
