use crate::prelude::*;

/// Various statistics about this Modrinth instance.
#[derive(Debug, Clone, Deserialize)]
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

endpoint! {
    "GET" "v2/statistics" -> "Statistics";

    /// ### Various statistics about this Modrinth instance
    ///
    /// Get the number of projects, versions, version files, and authors on this Modrinth instance.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/statistics/) for more details.
    fn statistics() -> Statistics
}
