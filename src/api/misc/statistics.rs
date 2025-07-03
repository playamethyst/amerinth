use crate::prelude::*;

/// Various statistics about this Modrinth instance.
#[derive(Debug, Deserialize)]
pub struct Statistics {
    /// Number of projects on Modrinth
    pub projects: usize,
    /// Number of versions on Modrinth
    pub versions: usize,
    /// Number of version files on Modrinth
    pub files: usize,
    /// Number of authors (users with projects) on Modrinth
    pub authors: usize,
}

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/statistics", response = "Statistics")]
struct GetStatistics;

/// ### GET `/statistics`
/// Get various statistics about this Modrinth instance.
pub async fn statistics<State>(modrinth: &Modrinth<State>) -> Result<Statistics, ClientError>
where
    State: AuthState,
{
    Ok(GetStatistics.exec(&modrinth.client).await?.parse()?)
}
