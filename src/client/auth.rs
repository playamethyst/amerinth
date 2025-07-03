use crate::Modrinth;
use chrono::{DateTime, Utc};
use http::{HeaderValue, Request, Response};
use rustify::{Endpoint, errors::ClientError};

/// An authentication state for the Modrinth API.
pub trait AuthState: Send + Sync {}

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
pub struct Pat(pub(crate) String, pub(crate) DateTime<Utc>);
impl AuthState for Pat {}

impl Authenticated for Pat {
    fn header(&self) -> String {
        self.0.clone()
    }

    fn is_valid(&self) -> bool {
        self.1 > Utc::now()
    }
}

/// Middleware to insert authentication into Rustify clients.
pub struct AuthMiddleware<'a, Auth>(pub(crate) &'a Modrinth<Auth>)
where
    Auth: AuthState;

impl rustify::MiddleWare for AuthMiddleware<'_, Unauthenticated> {
    fn request<E: Endpoint>(&self, _: &E, _: &mut Request<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }

    fn response<E: Endpoint>(&self, _: &E, _: &mut Response<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }
}

impl<Auth> rustify::MiddleWare for AuthMiddleware<'_, Auth>
where
    Auth: Authenticated,
{
    fn request<E: Endpoint>(
        &self,
        endpoint: &E,
        req: &mut Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        // todo: check if the authorization is still valid first

        let url = endpoint.url(&self.0.client.base)?.to_string();
        let auth_header =
            HeaderValue::from_str(self.0.auth.header().as_str()).map_err(|source| {
                ClientError::RequestBuildError {
                    source: source.into(),
                    method: endpoint.method(),
                    url,
                }
            })?;
        req.headers_mut().append("Authorization", auth_header);

        Ok(())
    }

    fn response<E: Endpoint>(&self, _: &E, _: &mut Response<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }
}
