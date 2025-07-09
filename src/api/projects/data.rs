use crate::prelude::*;

#[cfg(any(feature = "projects", feature = "tags"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSide {
    /// The project is required to function.
    Required,
    /// The project is not required, but may enhance the experience.
    Optional,
    /// The project is unsupported, meaning it is not recommended for use.
    Unsupported,
    /// It is unknown whether the project will work.
    Unknown,
}

#[cfg(any(feature = "projects", feature = "tags"))]
vec_enum! {
    /// Types of projects that can be found on Modrinth.
    #[derive(EnumString, Hash, PartialEq, Eq)]
    #[strum(serialize_all = "lowercase")]
    enum ProjectType {
        /// A mod is a modification to the game that adds new features, mechanics, or content.
        Mod,
        /// A modpack is a curated collection of mods that are designed to work together.
        ModPack,
        /// A resource pack is a collection of assets that change the game's visuals or sounds.
        ResourcePack,
        /// A shader is a special type of resource pack that enhances the game's graphics with advanced visual effects.
        Shader,
        /// A plugin is a server-side modification that adds new features or functionality to the game.
        Plugin,
        /// A data pack is a collection of data-driven content that modifies or adds to the game's mechanics.
        DataPack,

        #[strum(disabled)]
        Other(String),
    }
    vec_meta(
        #[derive(Hash, PartialEq, Eq)]
    )
}

#[cfg(any(feature = "projects", feature = "tags"))]
deserialize_other!(ProjectType);

#[cfg(any(feature = "projects", feature = "tags"))]
impl<'de> Deserialize<'de> for ProjectTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // filter out the always present "project" type
        let all_types: Vec<ProjectType> = Vec::deserialize(deserializer)?;
        let filtered = all_types
            .into_iter()
            .filter(|pt| !matches!(pt, ProjectType::Other(s) if s == "project"))
            .collect();
        Ok(Self(filtered))
    }
}
