use crate::prelude::*;
use std::collections::HashMap;

/// Mod version information that can be consumed by Forge's update checker
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
pub enum ForgeUpdatesError {
    /// The Forge updates file for the given project does not exist.
    #[error(r#"Project "{0}" does not exist."#)]
    DoesNotExist(String),
    #[error(transparent)]
    Client(#[from] ClientError),
}

/// ### GET `/updates/{id|slug}/forge_updates.json`
/// Forge Updates JSON file
pub async fn forge<State>(
    modrinth: &Modrinth<State>,
    query: impl Into<String>,
) -> Result<ForgeUpdates, ForgeUpdatesError>
where
    State: AuthState,
{
    let query = query.into();
    let req = GetForgeUpdates {
        query: query.clone(),
    };
    match req.exec(&modrinth.client).await {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(ForgeUpdatesError::DoesNotExist(query)),
    }
}
