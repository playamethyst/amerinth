use crate::prelude::*;
use crate::projects::{ProjectSide, ProjectType};

use_all!(pub donation);
use_all!(pub license);
use_all!(pub loader);
use_all!(pub(crate) macros);

tag! {
    "v2/tag/category";

    /// ### Get a list of categories
    ///
    /// Gets an array of categories, their icons, and applicable project types.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/categorylist/) for more details.
    pub fn categories() -> Category;

    /// A category that can be applied to a project
    ["name": String] -> {
        /// The SVG icon of a category
        icon: String,
        /// The project type this category is applicable to
        project_type: ProjectType,
        /// The header under which the category should go
        header: CategoryHeader
    }
}

other_enum! {
    /// Headers that categories can be grouped under.
    #[strum(serialize_all = "lowercase")]
    pub enum CategoryHeader {
        /// Categories that are related to the project type.
        Categories,
        /// Categories that are related to the features of the project.
        Features,
        /// Categories that are related to the resolution(s) of textures in the project.
        Resolutions,
        /// Categories that are related to the performance impact of the project.
        PerformanceImpact,
    }

    Other(String)
}

tag! {
    "v2/tag/game_version";

    /// ### Get a list of game versions
    ///
    /// Gets an array of game versions and information about them.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/versionlist/) for more details.
    pub fn game_versions() -> GameVersion;

    /// A version of a game that a project can target.
    ["version": String] -> {
        /// The type of the game version
        version_type: GameVersionType,
        /// The date of the game version release
        date: DateTime<Utc>,
        /// Whether or not this is a major version, used for Featured Versions
        major: bool
    }
}

/// The type of release a game version is.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Alpha,
    Beta,
    Snapshot,
    Release,
}

tag_vec! {
    /// ### Get a list of project types
    ///
    /// Gets an array of valid project types.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/projecttypelist/) for more details.
    pub project_types, ProjectType ("Vec<ProjectType>"), "v2/tag/project_type";
}

tag_vec! {
    /// ### Get a list of project sides
    ///
    /// Gets an array of valid project sides.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/sidetypelist/) for more details.
    pub side_types, ProjectSide ("Vec<ProjectSide>"), "v2/tag/side_type";
}

tag_vec! {
    /// ### Get a list of report types
    ///
    /// Gets an array of valid report types.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/reporttypelist/) for more details.
    pub report_types, ReportType ("Vec<ReportType>"), "v2/tag/report_type";
}

/// A report type supported by Modrinth.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportType {
    /// The project is spam or contains spam content.
    Spam,
    /// The project violates copyright laws or contains copyrighted material without permission.
    Copyright,
    /// The project is inappropriate or contains offensive content.
    Inappropriate,
    /// The project is malicious or contains harmful content.
    Malicious,
    /// The project is [cybersquatting](https://en.wikipedia.org/wiki/Cybersquatting) the name.
    #[serde(rename = "name-squatting")]
    NameSquatting,
    /// The project has a poor description.
    #[serde(rename = "poor description")]
    PoorDescription,
    /// The project has invalid metadata, such as missing or incorrect fields.
    #[serde(rename = "invalid metadata")]
    InvalidMetadata,
    /// There is another issue with the project not covered by the other report types.
    Other,
}
