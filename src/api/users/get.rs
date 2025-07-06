use std::collections::HashMap;

use super::User;
use crate::prelude::*;

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user/{self.user}", response = "User")]
struct GetUser {
    #[endpoint(skip)]
    user: String,
}

/// ### GET `/user/{id|username}`
/// Get a user
pub async fn get<Auth: Authenticated>(
    modrinth: &Modrinth<Auth>,
    user: impl Into<String>,
) -> Result<User> {
    let user = user.into();
    match exec!(GetUser { user: user.clone() }, modrinth) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(Error::NotFound {
            resource: "user",
            id: user,
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

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/users", response = "Vec<User>")]
struct GetUsers {
    #[endpoint(query)]
    ids: EndpointVec<String>,
}

/// GET `/users`
/// Get multiple users
pub async fn get_many<Auth, T>(
    modrinth: &Modrinth<Auth>,
    users: Vec<T>,
) -> Result<HashMap<T, Option<User>>>
where
    Auth: AuthState,
    T: Clone + std::hash::Hash + Into<String> + PartialEq + Eq,
{
    let resolved: Vec<User> = exec!(
        GetUsers {
            ids: users.clone().into_iter().map(|u| u.into()).collect(),
        },
        modrinth
    )?
    .parse()?;
    let mut out = HashMap::new();

    for user in users {
        let s = user.clone().into();
        if !resolved.iter().any(|u| u.username == s) {
            out.insert(user, None);
        } else {
            let user_data = resolved.iter().find(|u| u.username == s);
            out.insert(user, user_data.cloned());
        }
    }

    Ok(out)
}
