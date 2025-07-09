use crate::prelude::*;

endpoint! {
    "GET" "v2/tag/license/{self.id}" {
        #[endpoint(skip)]
        id: String [license.into()]
    } -> "License";

    /// ### Get the text and title of a license
    ///
    /// Get the text and title of a license supported by Modrinth.
    /// Modrinth supports any license that is listed on the [SPDX License List](https://spdx.org/licenses/).
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/licensetext/) for more details.
    ///
    /// ### Arguments
    ///
    /// - `license` - The SPDX ID of the license (e.g., "MIT", "LGPL-3.0-or-later")
    ///
    /// ### Errors
    ///
    /// Returns [ModrinthError::NotFound] if the license does not exist.
    fn license(license: &str) -> License {
        |res| match res {
            Ok(res) => Ok(res.parse()?),
            Err(_) => Err(ModrinthError::NotFound {
                resource: "license",
                id: license.into(),
            })
        }
    }
}

/// A license supported by Modrinth.
#[derive(Debug, Clone, Deserialize)]
pub struct License {
    pub title: String,
    pub body: String,
}
