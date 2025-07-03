use crate::ModrinthError;
pub use auth::AuthState;
use auth::*;
use rustify::clients::reqwest::Client;
use time::{Date, Month};
pub use user_agent::UserAgent;

mod auth;
mod user_agent;

/// Authentication for the Modrinth API
pub struct Modrinth<State: AuthState> {
    state: State,
    staging: bool,
    pub(crate) client: Client,
}

fn new_modrinth(
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
        state: Unauthenticated,
        staging,
        client: Client::new(if staging {
            "https://staging-api.modrinth.com/v2"
        } else {
            "https://api.modrinth.com/v2"
        }, reqwest::Client::builder().user_agent(user_agent).build()?),
    })
}

impl Modrinth<Unauthenticated> {
    /// Create a new unauthenticated client that can communicate with the Modrinth API.
    ///
    /// ### User Agent
    /// The user agent allows the Modrinth API to uniquely identify your application,
    /// and let the Modrinth team contact you if necessary. While it is not required
    /// in the context of this library, it is highly recommended to provide a user agent.
    /// If one is not provided, a default user agent identifying `amerinth` will be used.
    pub fn new(user_agent: Option<UserAgent>) -> Result<Modrinth<Unauthenticated>, ModrinthError> {
        new_modrinth(false, user_agent)
    }

    /// Create a new unauthenticated client that can communicate with the Modrinth API in staging mode.
    ///
    /// See the documentation for [Modrinth::new] for more information about the [Modrinth] client.
    pub fn staging(
        user_agent: Option<UserAgent>,
    ) -> Result<Modrinth<Unauthenticated>, ModrinthError> {
        new_modrinth(true, user_agent)
    }

    /// Authenticate a Modrinth client with a [Personal Access Token](https://modrinth.com/settings/pats) (PAT).
    pub fn pat(self, token: String) -> Modrinth<PAT> {
        Modrinth {
            state: PAT(token, None),
            staging: self.staging,
            client: self.client,
        }
    }

    /// Authenticate a Modrinth client with a [Personal Access Token](https://modrinth.com/settings/pats) (PAT)
    /// that expires on a specific date.
    pub fn pat_expires(
        self,
        token: String,
        day: u8,
        month: u8,
        year: i32,
    ) -> Result<Modrinth<PAT>, ModrinthError> {
        // figure out the expiration date
        let month = Month::try_from(month)?;
        let date = Date::from_calendar_date(year, month, day)?;
        let expires_at = date.with_hms(23, 59, 59)?.assume_utc();

        Ok(Modrinth {
            state: PAT(token, Some(expires_at)),
            staging: self.staging,
            client: self.client,
        })
    }
}
