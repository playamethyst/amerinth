use crate::helpers::enum_vec;
use serde::{Deserialize, Deserializer};

enum_vec! {
    /// A modrinth project type.
    enum ProjectType {
        Mod,
        ModPack,
        ResourcePack,
        Shader,
        Plugin,
        DataPack,
        Other(String),
    }
}

impl<'de> Deserialize<'de> for ProjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "mod" => ProjectType::Mod,
            "modpack" => ProjectType::ModPack,
            "resourcepack" => ProjectType::ResourcePack,
            "shader" => ProjectType::Shader,
            "plugin" => ProjectType::Plugin,
            "datapack" => ProjectType::DataPack,
            unknown => ProjectType::Other(unknown.to_string()),
        })
    }
}

impl<'de> Deserialize<'de> for ProjectTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
