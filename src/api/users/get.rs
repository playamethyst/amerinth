use super::User;
use crate::prelude::*;
use std::{collections::HashMap, hash::Hash};

/// ### Get a user
/// Get a user by their username or ID.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/getuser/) for more details.
///
/// ### Arguments
///
/// - `user` - The username or ID of the user
///
/// ### Errors
///
/// Returns [ModrinthError::NotFound] if the user does not exist.
pub async fn get<Auth: Authenticated>(
    modrinth: &Modrinth<Auth>,
    user: impl Into<String>,
) -> Result<User, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(method = "GET", path = "v2/user/{self.user}", response = "User")]
    struct GetUser {
        #[endpoint(skip)]
        user: String,
    }

    let user = user.into();
    match exec!(GetUser { user: user.clone() }, modrinth) {
        Ok(res) => Ok(res.parse()?),
        Err(_) => Err(ModrinthError::NotFound {
            resource: "user",
            id: user,
        }),
    }
}

/// ### Get user from authorization header
///
/// Gets the currently authenticated user.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/getuserfromauth/) for more details.
///
/// ### Errors
///
/// Returns [ModrinthError::Unauthorized] if the user is not authenticated.
pub async fn current<Auth: Authenticated>(
    modrinth: &Modrinth<Auth>,
) -> Result<User, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(method = "GET", path = "v2/user", response = "User")]
    struct GetCurrentUser;

    Ok(exec!(GetCurrentUser, modrinth)?.parse()?)
}

/// ### Get multiple users
///
/// Get multiple users by their usernames or IDs.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/getusers/) for more details.
///
/// ### Arguments
///
/// - `users` - A [Vec] of usernames or IDs of the users
///
/// ### Errors
///
/// Returns [ModrinthError::NotFound] if any of the users do not exist.
pub async fn many<Auth, T>(
    modrinth: &Modrinth<Auth>,
    users: Vec<T>,
) -> Result<HashMap<T, Option<User>>, ModrinthError>
where
    Auth: AuthState,
    T: Clone + Hash + Into<String> + PartialEq + Eq,
{
    #[derive(Endpoint)]
    #[endpoint(method = "GET", path = "v2/users", response = "Vec<User>")]
    struct GetUsers {
        #[endpoint(query)]
        ids: EndpointVec<String>,
    }

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
