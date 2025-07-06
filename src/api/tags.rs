use crate::api::projects::ProjectTypes;
use crate::prelude::*;
use pastey::paste;

macro_rules! tag {
    (@
        $(#[$fn_meta:meta])*
        $fn:ident, $response:expr, $endpoint:expr, $return:ty
    ) => {
        paste! {
            #[derive(Endpoint)]
            #[endpoint(method = "GET", path = $endpoint, response = $response)]
            #[allow(non_camel_case_types)]
            struct [<Get $fn>];

            $(#[$fn_meta])*
            pub async fn $fn<Auth: AuthState>(modrinth: &Modrinth<Auth>) -> Result<Vec<$return>> {
                Ok(exec!([<Get $fn>], modrinth)?.parse()?)
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
        struct $struct:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        tag!(@ $(#[$fn_meta])* $fn, $response, $endpoint, $struct);
        #[derive(Debug, Deserialize)]
        pub struct $struct {
            $(
                $(#[$field_meta])*
                pub $field: $type,
            )*
        }
    };
}

tag! {
    /// ### GET `/tag/category`
    /// Get a list of categories
    categories, "Vec<Category>", "v2/tag/category",
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

tag! {
    /// ### GET `/tag/loader`
    /// Get a list of loaders
    loaders, "Vec<Loader>", "v2/tag/loader",
    struct Loader {
        /// The SVG icon of a loader
        icon: String,
        /// The name of the loader
        name: String,
        /// The project types that this loader is applicable to
        supported_project_types: ProjectTypes
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Release,
    Snapshot,
    Alpha,
    Beta,
}

tag! {
    /// ### GET `/tag/game_version`
    /// Get a list of game versions
    game_versions, "Vec<GameVersion>", "v2/tag/game_version",
    struct GameVersion {
        /// The name/number of the game version
        version: String,
        /// The type of the game version
        version_type: GameVersionType,
        /// The date of the game version release
        date: Date,
        /// Whether or not this is a major version, used for Featured Versions
        major: bool,
    }
}
