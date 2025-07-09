use crate::prelude::*;
use std::collections::HashMap;

/// Mod version information that can be consumed by Forge's update checker.
#[derive(Debug, Clone, Deserialize)]
pub struct ForgeUpdates {
    /// A link to the mod page
    pub homepage: String,
    /// A list of the recommended and latest versions for each Minecraft release
    pub promos: HashMap<String, Promo>,
}

/// The recommended and latest versions for a Minecraft release.
#[derive(Debug, Clone, Deserialize)]
pub struct Promo {
    /// The mod version that is recommended for {version}. Excludes versions with the alpha and beta version types.
    pub recommended: String,
    /// The latest mod version for {version}. Shows versions with the alpha and beta version types.
    pub latest: String,
}

// #[derive(Endpoint)]
// #[endpoint(
//     method = "GET",
//     path = "updates/{self.project}/forge_updates.json",
//     response = "ForgeUpdates"
// )]
// struct GetForgeUpdates {
//     #[endpoint(skip)]
//     project: String,
// }

endpoint! {
    "GET" "updates/{self.project}/forge_updates.json" {
        #[endpoint(skip)]
        project: String [project.into()]
    } -> "ForgeUpdates";

    /// ### Forge Updates JSON file
    ///
    /// Get mod version information in the format consumed by Forge's update checker.
    /// This includes the recommended and latest versions for each Minecraft release.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/forgeupdates/) for more details.
    ///
    /// ### Arguments
    ///
    /// - `project` - The ID or slug of the project
    ///
    /// ### Errors
    ///
    /// Returns [ModrinthError::NotFound] if the project does not exist.
    fn forge(project: &str) -> ForgeUpdates {
        |res| match res {
            Ok(res) => Ok(res.parse()?),
            Err(_) => Err(ModrinthError::NotFound {
                resource: "project",
                id: project.into()
            })
        }
    }
}
