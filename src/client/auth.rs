use crate::Modrinth;
use chrono::{DateTime, Utc};
use http::{HeaderValue, Request, Response};
use rustify::{Endpoint, errors::ClientError};

/// The authentication state of the client.
pub trait AuthState: Send + Sync {
    /// Authenticate a request if applicable.
    fn auth<E: Endpoint>(
        &self,
        endpoint: &E,
        req: &mut Request<Vec<u8>>,
        base_url: &str,
    ) -> Result<(), ClientError>;
}

/// The client is unauthenticated.
pub struct Unauthenticated;
/// The client is authenticated.
pub trait Authenticated: AuthState {}

impl AuthState for Unauthenticated {
    fn auth<E: Endpoint>(
        &self,
        _: &E,
        _: &mut Request<Vec<u8>>,
        _: &str,
    ) -> Result<(), ClientError> {
        // unauthenticated clients do not need to do anything
        Ok(())
    }
}

/// The client is [Authenticated] using a [Personal Access Token](https://modrinth.com/settings/pats).
pub struct Pat(pub(crate) String, pub(crate) DateTime<Utc>);
impl Authenticated for Pat {}

impl AuthState for Pat {
    fn auth<E: Endpoint>(
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
        self.0.auth.auth(endpoint, req, &self.0.client.base)
    }

    fn response<E: Endpoint>(&self, _: &E, _: &mut Response<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }
}
