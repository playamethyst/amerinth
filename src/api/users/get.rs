use super::User;
use crate::prelude::*;

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user/{self.query}", response = "User")]
struct GetUser {
    query: String,
}

/// ### GET `/user/{id|username}`
/// Get a user
pub async fn get<Auth: Authenticated>(
    modrinth: &Modrinth<Auth>,
    query: impl Into<String>,
) -> Result<User> {
    let query = query.into();
    match exec!(
        GetUser {
            query: query.clone(),
        },
        modrinth
    ) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(Error::NotFound {
            resource: "user",
            id: query,
        }),
    }
}

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user", response = "User")]
struct GetCurrentUser;

/// ### GET `/user`
/// Get user from authorization header
pub async fn current<Auth: Authenticated>(modrinth: &Modrinth<Auth>) -> Result<User> {
    match exec!(GetCurrentUser, modrinth) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(Error::Unauthorized),
    }
}
