use reqwest::Client;

use crate::prelude::*;
use std::collections::HashMap;

/// Mod version information that can be consumed by Forge’s update checker
#[derive(Debug, Deserialize)]
pub struct ForgeUpdates {
    /// A link to the mod page
    pub homepage: String,
    /// A list of the recommended and latest versions for each Minecraft release
    pub promos: HashMap<String, Promo>,
}

/// The recommended and latest versions for a Minecraft release
#[derive(Debug, Deserialize)]
pub struct Promo {
    /// The mod version that is recommended for {version}. Excludes versions with the alpha and beta version types.
    pub recommended: String,
    /// The latest mod version for {version}. Shows versions with the alpha and beta version types.
    pub latest: String,
}

#[derive(Endpoint)]
#[endpoint(
    method = "GET",
    path = "updates/{self.query}/forge_updates.json",
    response = "ForgeUpdates"
)]
pub struct GetForgeUpdates {
    #[endpoint(skip)]
    query: String,
}

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error(r#"Project "{0}" does not exist."#)]
    DoesNotExist(String),
    #[error(transparent)]
    Other(#[from] ClientError),
}

/// ### GET `/updates/{id|slug}/forge_updates.json`
/// Forge Updates JSON file
pub async fn forge<State>(
    modrinth: &Modrinth<State>,
    query: impl Into<String>,
) -> Result<ForgeUpdates, ForgeError>
where
    State: AuthState,
{
    let query = query.into();
    let req = GetForgeUpdates {
        query: query.clone(),
    };
    match req.exec(&modrinth.client).await {
        Ok(res) => Ok(res.parse()?),
        // this endpoint only has one possible error - does not exist
        // see: https://github.com/modrinth/code/blob/9dc56442644d8bd8ad2eb67f07cb2763b3a2fe30/apps/labrinth/src/routes/updates.rs#L41
        Err(_) => Err(ForgeError::DoesNotExist(query)),
    }
}
