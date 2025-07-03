use time::OffsetDateTime;

/// An authentication state for the Modrinth API.
pub trait AuthState {}

/// The unauthenticated state of the Modrinth client.
pub struct Unauthenticated;
impl AuthState for Unauthenticated {}

/// Generic trait for authenticated states in the Modrinth API.
pub trait Authenticated: AuthState {
    /// Get the authorization header.
    fn header(&self) -> String;

    /// Check if the authentication is still valid.
    fn is_valid(&self) -> bool;
}

/// Authentication using a [Personal Access Token](https://modrinth.com/settings/pats).
pub struct PAT(pub(crate) String, pub(crate) Option<OffsetDateTime>);
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
