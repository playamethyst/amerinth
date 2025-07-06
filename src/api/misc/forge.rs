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

/// ### GET `/updates/{id|slug}/forge_updates.json`
/// Forge Updates JSON file
pub async fn forge<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
    query: impl Into<String>,
) -> Result<ForgeUpdates> {
    let query = query.into();
    match exec!(
        GetForgeUpdates {
            query: query.clone(),
        },
        modrinth
    ) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(Error::NotFound {
            resource: "project",
            id: query,
        }),
    }
}
