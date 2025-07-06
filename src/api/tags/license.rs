use crate::prelude::*;

/// A license supported by Modrinth.
#[derive(Debug, Clone, Deserialize)]
pub struct License {
    pub title: String,
    pub body: String,
}
use crate::Modrinth;

/// ### Get the text and title of a license
///
/// Get the text and title of a license supported by Modrinth.
/// Modrinth supports any license that is listed on the [SPDX License List](https://spdx.org/licenses/).
///
/// ### Arguments
///
/// - `license` - The SPDX ID of the license (e.g., "MIT", "LGPL-3.0-or-later")
///
/// ### Errors
///
/// Returns [ModrinthError::NotFound] if the license does not exist.
pub async fn license<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
    license: impl Into<String>,
) -> Result<License, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(
        method = "GET",
        path = "v2/tag/license/{self.id}",
        response = "License"
    )]
    struct GetLicense {
        #[endpoint(skip)]
        id: String,
    }

    let license = license.into();
    match exec!(
        GetLicense {
            id: license.clone()
        },
        modrinth
    ) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(ModrinthError::NotFound {
            resource: "license",
            id: license,
        }),
    }
}
