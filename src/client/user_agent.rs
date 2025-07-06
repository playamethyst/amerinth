use std::fmt;

use bon::Builder;

/// Create a user agent optimised for the Modrinth API.
/// You can view their guidelines for user agents [here](https://docs.modrinth.com/api/#user-agents).
///
/// Try to fill out as many fields as possible to avoid having your traffic blocked.
#[derive(Builder)]
pub struct UserAgent<'a> {
    /// The name of the project, e.g. `amerinth`.
    #[builder(start_fn)]
    project_name: &'a str,
    /// The version of the crate, e.g. `0.1.0`.
    version: Option<&'a str>,
    /// The author of the project, e.g. `getamethyst`.
    /// This should ideally be a GitHub username.
    author: Option<&'a str>,
    /// The contact information for the project.
    /// This is ideally an email address or a website.
    contact: Option<&'a str>,
}

impl fmt::Display for UserAgent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.author.map(|a| format!("{a}/")).unwrap_or_default(),
            self.project_name,
            self.version.map(|v| format!("/{v}")).unwrap_or_default(),
            self.contact.map(|c| format!(" ({c})")).unwrap_or_default()
        )
    }
}
