use crate::prelude::*;

use_all!(pub loader);

macro_rules! tag {
    (@
        $(#[$fn_meta:meta])*
        $fn:ident, $response:expr, $endpoint:expr, $return:ty
    ) => {
        pastey::paste! {
            $(#[$fn_meta])*
            pub async fn $fn<Auth: $crate::client::AuthState>(modrinth: &$crate::Modrinth<Auth>) -> Result<Vec<$return>, $crate::ModrinthError> {
                #[derive(rustify_derive::Endpoint)]
                #[endpoint(method = "GET", path = $endpoint, response = $response)]
                #[allow(non_camel_case_types)]
                struct [<Get $fn>];

                Ok($crate::helpers::exec!([<Get $fn>], modrinth)?.parse()?)
            }
        }
    };
    (
        $(#[$fn_meta:meta])*
        $fn:ident, $response:expr, $endpoint:expr, $return:ty
    ) => {
        tag!(@ $(#[$fn_meta])* $fn, $response, $endpoint, $return);
    };
    (
        $(#[$fn_meta:meta])*
        $fn:ident, $response:expr, $endpoint:expr,
        $(#[$struct_meta:meta])*
        struct $struct:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        $crate::tags::tag!(@ $(#[$fn_meta])* $fn, $response, $endpoint, $struct);
        $(#[$struct_meta])*
        #[derive(Debug, Clone, serde::Deserialize)]
        pub struct $struct {
            $(
                $(#[$field_meta])*
                pub $field: $type,
            )*
        }
    };
}
pub(crate) use tag;

tag! {
    /// ### Get a list of categories
    ///
    /// Gets an array of categories, their icons, and applicable project types.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/categorylist/) for more details.
    categories, "Vec<Category>", "v2/tag/category",

    /// A category is a way to group projects together based on their type or purpose.
    struct Category {
        /// The SVG icon of a category
        icon: String,
        /// The name of the category
        name: String,
        /// The project type this category is applicable to
        project_type: String,
        /// The header under which the category should go
        header: String,
    }
}

/// The type of game version.
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
    game_versions, "Vec<GameVersion>", "v2/tag/game_version",
    struct GameVersion {
        /// The name/number of the game version
        version: String,
        /// The type of the game version
        version_type: GameVersionType,
        /// The date of the game version release
        date: DateTime<Utc>,
        /// Whether or not this is a major version, used for Featured Versions
        major: bool,
    }
}
