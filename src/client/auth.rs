use crate::Modrinth;
use chrono::{DateTime, Utc};
use http::{HeaderMap, HeaderValue, Request, Response};
use rustify::{Endpoint, enums::RequestMethod, errors::ClientError};

/// The authentication state of the client.
pub trait AuthState: Send + Sync {
    /// Authenticate a request if applicable.
    fn auth(
        &self,
        method: RequestMethod,
        url: String,
        headers: &mut HeaderMap<HeaderValue>,
    ) -> Result<(), ClientError>;
}

/// The client is unauthenticated.
pub struct Unauthenticated;
/// The client is authenticated.
pub trait Authenticated: AuthState {}

impl AuthState for Unauthenticated {
    fn auth(
        &self,
        _: RequestMethod,
        _: String,
        _: &mut HeaderMap<HeaderValue>,
    ) -> Result<(), ClientError> {
        // unauthenticated clients do not need to do anything
        Ok(())
    }
}

/// The client is [Authenticated] using a [Personal Access Token](https://modrinth.com/settings/pats).
pub struct Pat(pub(crate) String, pub(crate) DateTime<Utc>);
impl Authenticated for Pat {}

fn header_string(
    method: RequestMethod,
    url: String,
    str: &str,
) -> Result<HeaderValue, ClientError> {
    HeaderValue::from_str(&str).map_err(|source| ClientError::RequestBuildError {
        source: source.into(),
        method,
        url,
    })
}

impl AuthState for Pat {
    fn auth(
        &self,
        method: RequestMethod,
        url: String,
        headers: &mut HeaderMap<HeaderValue>,
    ) -> Result<(), ClientError> {
        // todo: check if valid first

        // apply authorization header
        headers.append("Authorization", header_string(method, url, &self.0)?);

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
        let headers = req.headers_mut();
        let method = endpoint.method();
        let url = endpoint.url(&self.0.client.base)?.to_string();

        // apply user agent
        headers.insert(
            "User-Agent",
            header_string(method.clone(), url.clone(), &self.0.user_agent)?,
        );

        // apply authorization header
        self.0.auth.auth(method, url, headers)
    }

    fn response<E: Endpoint>(&self, _: &E, _: &mut Response<Vec<u8>>) -> Result<(), ClientError> {
        Ok(())
    }
}
