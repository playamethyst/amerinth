use crate::Modrinth;
use chrono::{DateTime, Utc};
use http::{HeaderValue, Request, Response};
use rustify::{Endpoint, errors::ClientError};

/// A client authentication state.
pub trait AuthState: Send + Sync {
    /// Apply authentication to a request if applicable.
    fn apply_auth<E: Endpoint>(
        &self,
        endpoint: &E,
        req: &mut Request<Vec<u8>>,
        base_url: &str,
    ) -> Result<(), ClientError>;
}

/// The client is unauthenticated.
pub struct Unauthenticated;

impl AuthState for Unauthenticated {
    fn apply_auth<E: Endpoint>(
        &self,
        _: &E,
        _: &mut Request<Vec<u8>>,
        _: &str,
    ) -> Result<(), ClientError> {
        Ok(())
    }
}

/// Shared functionality between authenticated clients.
pub trait Authenticated: AuthState {}

/// The client is authenticated using a [Personal Access Token](https://modrinth.com/settings/pats).
pub struct Pat(pub(crate) String, pub(crate) DateTime<Utc>);
impl Authenticated for Pat {}

impl AuthState for Pat {
    fn apply_auth<E: Endpoint>(
        &self,
        endpoint: &E,
        req: &mut Request<Vec<u8>>,
        base_url: &str,
    ) -> Result<(), ClientError> {
        // todo: check if valid first

        // apply authorization header
        let url = endpoint.url(base_url)?.to_string();
        let auth_header = HeaderValue::from_str(&self.0.clone()).map_err(|source| {
            ClientError::RequestBuildError {
                source: source.into(),
                method: endpoint.method(),
                url,
            }
        })?;
        req.headers_mut().append("Authorization", auth_header);

        Ok(())
    }
}

/// Insert authentication information into Rustify clients.
pub struct AuthMiddleware<'a, Auth>(pub(crate) &'a Modrinth<Auth>)
where
    Auth: AuthState;

impl<Auth: AuthState> rustify::MiddleWare for AuthMiddleware<'_, Auth> {
    fn request<E: Endpoint>(
        &self,
        endpoint: &E,
        req: &mut Request<Vec<u8>>,
    ) -> Result<(), ClientError> {
        self.0.auth.apply_auth(endpoint, req, &self.0.client.base)
    }

    fn response<E: Endpoint>(&self, _: &E, _: &mut Response<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }
}
