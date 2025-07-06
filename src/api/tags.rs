use crate::prelude::*;
use crate::projects::{ProjectSide, ProjectType};

use_all!(pub license);
use_all!(pub loader);
use_all!(pub report);

macro_rules! tag {
    (@ $key_ty:ty, $key:expr, $val_ty:ty, $val_key:expr) => {
        #[derive(serde::Deserialize)]
        struct Response {
            #[serde(rename = $key)]
            key: $key_ty,
            #[serde(rename = $val_key)]
            data: $val_ty
        }
    };
    (@ $key_ty:ty, $key:expr, $val_ty:ty) => {
        #[derive(serde::Deserialize)]
        struct Response {
            #[serde(rename = $key)]
            key: $key_ty,
            #[serde(flatten)]
            data: $val_ty
        }
    };
    (
        $(#[$fn_meta:meta])*
        $fn:ident, ($key:expr => $key_ty:ty), $endpoint:expr;
        $val_ty:ty $([$val_key:expr])?
    ) => {
        pastey::paste! {
            $(#[$fn_meta])*
            pub async fn $fn<Auth: $crate::client::AuthState>(
                    modrinth: &$crate::Modrinth<Auth>
            ) -> Result<std::collections::HashMap<$key_ty, $val_ty>, $crate::ModrinthError> {
                use rustify::Endpoint;

                $crate::tags::tag!(@ $key_ty, $key, $val_ty $(, $val_key)?);

                #[derive(rustify_derive::Endpoint)]
                #[endpoint(method = "GET", path = $endpoint, response = "Vec<Response>")]
                struct Request;

                let list: Vec<Response> = $crate::helpers::exec!(Request, modrinth)?.parse()?;

                Ok(list
                    .into_iter()
                    .map(|item| (item.key, item.data))
                    .collect()
                )
            }
        }
    };
    (
        $(#[$fn_meta:meta])*
        $fn:ident, $tag:ident, ($key:expr => $key_ty:ty), $endpoint:expr;
        $(#[$struct_meta:meta])*
        {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        pastey::paste! {
            $(#[$struct_meta])*
            #[derive(Debug, Clone, serde::Deserialize)]
            pub struct [<$tag Info>] {
                $(
                    $(#[$field_meta])*
                    pub $field: $type,
                )*
            }

            $crate::tags::tag!(
                $(#[$fn_meta])*
                $fn, ($key => $key_ty), $endpoint;
                [<$tag Info>]
            );
        }
    };
}
pub(crate) use tag;

/// Headers that categories can be grouped under.
#[derive(Debug, Clone, EnumString)]
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
    /// A custom or unknown header, represented by a string.
    Other(String),
}
deserialize_other!(CategoryHeader);

tag! {
    /// ### Get a list of categories
    ///
    /// Gets an array of categories, their icons, and applicable project types.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/categorylist/) for more details.
    categories, Category, ("name" => String), "v2/tag/category";
    /// A category that can be applied to a project
    {
        /// The SVG icon of a category
        icon: String,
        /// The project type this category is applicable to
        project_type: ProjectType,
        /// The header under which the category should go
        header: CategoryHeader,
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

tag! {
    /// ### Get a list of game versions
    ///
    /// Gets an array of game versions and information about them.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/versionlist/) for more details.
    game_versions, GameVersion, ("version" => String), "v2/tag/game_version";
    /// A version of a game that a project can target.
    {
        /// The type of the game version
        version_type: GameVersionType,
        /// The date of the game version release
        date: DateTime<Utc>,
        /// Whether or not this is a major version, used for Featured Versions
        major: bool,
    }
}

tag! {
    /// ### Get a list of donation platforms
    ///
    /// Gets an array of donation platforms and information about them.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/donationplatformlist/) for more details.
    donation_platforms, ("short" => String), "v2/tag/donation_platform";
    String ["name"]
}

/// ### Get a list of project types
///
/// Gets an array of valid project types.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/projecttypelist/) for more details.
pub async fn project_types<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
) -> Result<Vec<ProjectType>, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(
        method = "GET",
        path = "v2/tag/project_type",
        response = "Vec<ProjectType>"
    )]
    struct GetProjectTypes;

    Ok(exec!(GetProjectTypes, modrinth)?.parse()?)
}

/// ### Get a list of side types
///
/// Gets an array of valid side types.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/sidetypelist/) for more details.
pub async fn side_types<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
) -> Result<Vec<ProjectSide>, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(
        method = "GET",
        path = "v2/tag/side_type",
        response = "Vec<ProjectSide>"
    )]
    struct GetSideTypes;

    Ok(exec!(GetSideTypes, modrinth)?.parse()?)
}
