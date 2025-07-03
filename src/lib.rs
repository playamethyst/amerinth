// todo: endpoints
// - projects
// - versions
// - version-files
// - users
// - notifications
// - threads
// - teams
// - tags
// - misc

// todo: utils
// - mrpack downloader
// - curseforge -> modrinth

// todo: blocking apis

use bon::Builder;

/// Create a user agent optimised for the Modrinth API.
/// You can view their guidelines for user agents [here](https://docs.modrinth.com/api/#user-agents).
///
/// Try to fill out as many fields as possible to avoid being rate limited.
#[derive(Builder)]
pub struct UserAgent<'t> {
    /// The name of the project, e.g. `amerinth`.
    #[builder(start_fn)]
    project_name: &'t str,
    /// The version of the crate, e.g. `0.1.0`.
    version: Option<&'t str>,
    /// The author of the project, e.g. `getamethyst`.
    /// This should ideally be a GitHub username.
    author: Option<&'t str>,
    /// The contact information for the project.
    /// This is ideally an email address or a website.
    contact: Option<&'t str>,
}

impl ToString for UserAgent<'_> {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            self.author.map(|a| format!("{a}/")).unwrap_or_default(),
            self.project_name,
            self.version.map(|v| format!("/{v}")).unwrap_or_default(),
            self.contact.map(|c| format!(" ({c})")).unwrap_or_default()
        )
    }
}

pub struct ModrinthAuth {}

impl ModrinthAuth {
    /// Create a new client that can communicate with the Modrinth API.
    ///
    /// ### User Agent
    /// The user agent allows the Modrinth API to uniquely identify your application,
    /// and let the Modrinth team contact you if necessary. While it is not required
    /// in the context of this library, it is highly recommended to provide a user agent.
    /// If one is not provided, a default user agent identifying `amerinth` will be used.
    pub fn new(user_agent: Option<UserAgent>) -> Self {
        let user_agent = user_agent.unwrap_or({
            UserAgent::builder(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author("getamethyst")
                .contact("playamethyst.com")
                .build()
        });

        println!("{:?}", user_agent.to_string());

        Self {}
    }
}
