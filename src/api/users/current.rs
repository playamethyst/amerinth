use super::User;
use crate::prelude::*;

#[derive(Endpoint)]
#[endpoint(method = "GET", path = "v2/user", response = "User")]
struct GetCurrentUser;

pub async fn current<State>(modrinth: &Modrinth<State>) -> Result<User, ClientError>
where
    State: Authenticated,
{
    GetCurrentUser
        .with_middleware(&AuthMiddleware(modrinth))
        .exec(&modrinth.client)
        .await?
        .parse()
}
