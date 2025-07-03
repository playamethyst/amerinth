use time::{Date, Month, OffsetDateTime};
pub use user_agent::UserAgent;

mod user_agent;

/// Represents an unauthenticated client for the Modrinth API.
pub struct Unauthenticated;

/// Trait representing an authenticated client for the Modrinth API.
trait Authenticated {
    /// Get the authorization header.
    fn header(&self) -> String;

    /// Check if the authentication is still valid.
    fn is_valid(&self) -> bool;
}

/// Represents a client authenticated with a Personal Access Token (PAT).
#[derive(Debug)]
pub struct PAT(String, Option<OffsetDateTime>);

impl Authenticated for PAT {
    fn header(&self) -> String {
        format!("Bearer {}", self.0)
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
#[derive(Debug)]
pub struct ModrinthAuth<State> {
    state: State,
    user_agent: String,
}

impl ModrinthAuth<Unauthenticated> {
    /// Create a new unauthenticated client that can communicate with the Modrinth API.
    ///
    /// ### User Agent
    /// The user agent allows the Modrinth API to uniquely identify your application,
    /// and let the Modrinth team contact you if necessary. While it is not required
    /// in the context of this library, it is highly recommended to provide a user agent.
    /// If one is not provided, a default user agent identifying `amerinth` will be used.
    pub fn new(user_agent: Option<UserAgent>) -> ModrinthAuth<Unauthenticated> {
        let user_agent = user_agent
            .unwrap_or({
                UserAgent::builder(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("getamethyst")
                    .contact("playamethyst.com")
                    .build()
            })
            .to_string();

        ModrinthAuth {
            state: Unauthenticated,
            user_agent,
        }
    }

    pub fn pat(self, token: String) -> ModrinthAuth<PAT> {
        ModrinthAuth {
            state: PAT(token, None),
            user_agent: self.user_agent,
        }
    }

    pub fn pat_expires(
        self,
        token: String,
        day: u8,
        month: u8,
        year: i32,
    ) -> Result<ModrinthAuth<PAT>, time::error::ComponentRange> {
        let month = Month::try_from(month)?;
        let date = Date::from_calendar_date(year, month, day)?;
        let expires_at = date.with_hms(23, 59, 59)?.assume_utc();

        Ok(ModrinthAuth {
            state: PAT(token, Some(expires_at)),
            user_agent: self.user_agent,
        })
    }
}

#[allow(private_bounds)]
impl<T: Authenticated> ModrinthAuth<T> {}
