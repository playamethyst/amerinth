use crate::ModrinthError;
use auth::*;
pub use auth::{AuthMiddleware, AuthState, Authenticated};
use rustify::{clients::reqwest::Client, errors::ClientError};
use time::{Date, Month};
pub use user_agent::UserAgent;

mod auth;
mod user_agent;

/// Authentication for the Modrinth API
pub struct Modrinth<Auth>
where
    Auth: AuthState,
{
    auth: Auth,
    pub(crate) client: Client,
}

impl Modrinth<Unauthenticated> {
    /// Create a new unauthenticated client that can communicate with the Modrinth API.
    ///
    /// ### User Agent
    /// The user agent allows the Modrinth API to uniquely identify your application,
    /// and let the Modrinth team contact you if necessary. While it is not required
    /// in the context of this library, it is highly recommended to provide a user agent.
    /// If one is not provided, a default user agent identifying `amerinth` will be used.
    pub fn new(
        staging: bool,
        user_agent: Option<UserAgent>,
    ) -> Result<Modrinth<Unauthenticated>, ModrinthError> {
        let user_agent = user_agent
            .unwrap_or({
                UserAgent::builder(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("getamethyst")
                    .contact("playamethyst.com")
                    .build()
            })
            .to_string();

        Ok(Modrinth {
            auth: Unauthenticated,
            client: Client::new(
                if staging {
                    "https://staging-api.modrinth.com"
                } else {
                    "https://api.modrinth.com"
                },
                reqwest::Client::builder()
                    .user_agent(user_agent)
                    .build()
                    .map_err(|source| ClientError::ReqwestBuildError { source })?,
            ),
        })
    }

    /// Authenticate a Modrinth client with a [Personal Access Token](https://modrinth.com/settings/pats) (PAT).
    pub fn pat(
        self,
        token: String,
        day: u8,
        month: u8,
        year: i32,
    ) -> Result<Modrinth<Pat>, ModrinthError> {
        // encode the expiration date
        let month = Month::try_from(month)?;
        let date = Date::from_calendar_date(year, month, day)?;
        let expires_at = date.with_hms(23, 59, 59)?.assume_utc();

        Ok(Modrinth {
            auth: Pat(token, expires_at),
            client: self.client,
        })
    }
}
