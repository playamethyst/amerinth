use super::User;
use crate::prelude::*;
use std::{collections::HashMap, hash::Hash};

endpoint! {
    "GET" "v2/user/{self.user}" {
        #[endpoint(skip)]
        user: String [user.into()]
    } -> "User";

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
    pub fn get(user: &str) -> User {
        |res| match res {
            Ok(res) => Ok(res.parse()?),
            Err(_) => Err(ModrinthError::NotFound {
                resource: "user",
                id: user.into(),
            })
        }
    }
}

endpoint! {
    "GET" "v2/user" -> "User" [Authenticated];

    /// ### Get the currently authenticated user
    ///
    /// Gets the currently authenticated user from the Modrinth API.
    ///
    /// This endpoint requires authentication via an authorization header.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/getuserfromauth/) for more details.
    ///
    /// ### Errors
    ///
    /// Returns [ModrinthError::Unauthorized] if the user is not authenticated.
    pub fn current() -> User
}

endpoint! {
    "GET" "v2/users" {
        #[endpoint(query)]
        ids: DebugFmt<Vec<String>> [
            users.clone()
                .into()
                .into_iter()
                .map(|u| u.into())
                .collect::<Vec<_>>()
                .into()
        ]
    } -> "Vec<User>";

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
    pub fn many<T: [Clone + Hash + Into<String> + PartialEq + Eq]>(users: impl Clone + Into<Vec<T>>) -> HashMap<T, Option<User>> {
        |res| match res {
            Ok(res) => {
                let resolved: Vec<User> = res.parse()?;
                let mut out = HashMap::new();
                for user in users.into() {
                    let s = user.clone().into();
                    if !resolved.iter().any(|u| u.username == s) {
                        out.insert(user, None);
                    } else {
                        let user_data = resolved.iter().find(|u| u.username == s);
                        out.insert(user, user_data.cloned());
                    }
                }
                Ok(out)
            },
            Err(err) => Err(err.into())
        }
    }
}
