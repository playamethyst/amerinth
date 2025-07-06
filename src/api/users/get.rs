use super::User;
use crate::prelude::*;

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user/{self.query}", response = "User")]
struct GetUser {
    query: String,
}

#[derive(Debug, Error)]
pub enum GetUserError {
    /// The user with the given ID or username was not found.
    #[error(r#"User "{0}" not found."#)]
    NotFound(String),
    /// Client was not authorized to perform this request.
    #[error("Not authorized to perform this request.")]
    Unauthorized,
    #[error(transparent)]
    Client(#[from] ClientError),
}

/// ### GET `/user/{id|username}`
/// Get a user
pub async fn get<State>(
    modrinth: &Modrinth<State>,
    query: impl Into<String>,
) -> Result<User, GetUserError>
where
    State: Authenticated,
{
    let query = query.into();
    let req = GetUser {
        query: query.clone(),
    };
    match req
        .with_middleware(&AuthMiddleware(modrinth))
        .exec(&modrinth.client)
        .await
    {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(GetUserError::NotFound(query)),
    }
}

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user", response = "User")]
struct GetCurrentUser;

/// ### GET `/user`
/// Get user from authorization header
pub async fn current<State>(modrinth: &Modrinth<State>) -> Result<User, GetUserError>
where
    State: Authenticated,
{
    match GetCurrentUser
        .with_middleware(&AuthMiddleware(modrinth))
        .exec(&modrinth.client)
        .await
    {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(GetUserError::Unauthorized),
    }
}
