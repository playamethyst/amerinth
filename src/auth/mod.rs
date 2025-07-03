use reqwest::Client;
use time::{Date, Month, OffsetDateTime};
pub use user_agent::UserAgent;

mod user_agent;

/// An error that can occur when authenticating with the Modrinth API.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Failed to build HTTP client: {0}")]
    HttpBuild(#[from] reqwest::Error),
    #[error("Invalid date provided for expiration: {0}")]
    InvalidDate(#[from] time::error::ComponentRange),
}

/// An authentication state for the Modrinth API.
pub trait AuthState {}

/// The unauthenticated state of the Modrinth client.
pub struct Unauthenticated;
impl AuthState for Unauthenticated {}

/// Generic trait for authenticated states in the Modrinth API.
trait Authenticated: AuthState {
    /// Get the authorization header.
    fn header(&self) -> String;

    /// Check if the authentication is still valid.
    fn is_valid(&self) -> bool;
}

/// Authentication using a [Personal Access Token](https://modrinth.com/settings/pats).
pub struct PAT(String, Option<OffsetDateTime>);
impl AuthState for PAT {}

impl Authenticated for PAT {
    fn header(&self) -> String {
        self.0.clone()
    }

    fn is_valid(&self) -> bool {
        if let Some(expires_at) = self.1 {
            OffsetDateTime::now_utc() < expires_at
        } else {
            // if no expiration date is set, assume the token is valid indefinitely
            true
        }
    }
}

// todo: OAuth

/// Authentication for the Modrinth API.
pub struct ModrinthAuth<State: AuthState> {
    state: State,
    client: Client,
}

impl ModrinthAuth<Unauthenticated> {
    /// Create a new unauthenticated client that can communicate with the Modrinth API.
    ///
    /// ### User Agent
    /// The user agent allows the Modrinth API to uniquely identify your application,
    /// and let the Modrinth team contact you if necessary. While it is not required
    /// in the context of this library, it is highly recommended to provide a user agent.
    /// If one is not provided, a default user agent identifying `amerinth` will be used.
    pub fn new(user_agent: Option<UserAgent>) -> Result<ModrinthAuth<Unauthenticated>, AuthError> {
        let user_agent = user_agent
            .unwrap_or({
                UserAgent::builder(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("getamethyst")
                    .contact("playamethyst.com")
                    .build()
            })
            .to_string();

        Ok(ModrinthAuth {
            state: Unauthenticated,
            client: Client::builder().user_agent(user_agent).build()?,
        })
    }

    /// Authenticate a Modrinth client with a [Personal Access Token](https://modrinth.com/settings/pats) (PAT).
    pub fn pat(self, token: String) -> ModrinthAuth<PAT> {
        ModrinthAuth {
            state: PAT(token, None),
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
    ) -> Result<ModrinthAuth<PAT>, AuthError> {
        // figure out the expiration date
        let month = Month::try_from(month)?;
        let date = Date::from_calendar_date(year, month, day)?;
        let expires_at = date.with_hms(23, 59, 59)?.assume_utc();

        Ok(ModrinthAuth {
            state: PAT(token, Some(expires_at)),
            client: self.client,
        })
    }
}

#[allow(private_bounds)]
impl<T: Authenticated> ModrinthAuth<T> {}
