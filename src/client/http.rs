use super::{Modrinth, auth::*};
use crate::ModrinthError;
use serde::de::DeserializeOwned;

fn build_uri(staging: bool, endpoint: &str) -> String {
    if staging {
        format!("https://staging-api.modrinth.com/v2{}", endpoint)
    } else {
        format!("https://api.modrinth.com/v2{}", endpoint)
    }
}

#[allow(async_fn_in_trait)]
pub trait HttpClient {
    /// Send a GET request to the specified URL and deserialize the response into type `T`.
    async fn get<T>(&self, endpoint: &str) -> Result<T, ModrinthError>
    where
        T: DeserializeOwned;
}

impl HttpClient for Modrinth<Unauthenticated> {
    async fn get<T>(&self, endpoint: &str) -> Result<T, ModrinthError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(build_uri(self.staging, endpoint))
            .send()
            .await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }
}

impl<State: Authenticated> HttpClient for Modrinth<State> {
    async fn get<T>(&self, endpoint: &str) -> Result<T, ModrinthError>
    where
        T: DeserializeOwned,
    {
        if !self.state.is_valid() {
            return Err(ModrinthError::InvalidToken);
        }

        let response = self
            .client
            .get(build_uri(self.staging, endpoint))
            .header("Authorization", self.state.header())
            .send()
            .await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }
}
