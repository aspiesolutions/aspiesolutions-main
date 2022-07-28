use serde::Deserialize;

use crate::constants::{
    AUTH0_JWKS_DISCOVERY_ENDPOINT, ENV_KEY_AUTH0_AUDIENCE, ENV_KEY_AUTH0_CLIENT_ID,
    ENV_KEY_AUTH0_CLIENT_SECRET, ENV_KEY_AUTH0_DOMAIN,
};
#[derive(Deserialize, Debug)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    // the string used to identify this api. used to verfiy that tokens were meant for this api
    pub audience: String,
    // pub jwks_key_id: String,
}

impl Auth0Config {
    /// Constructs a new instance of this struct using std::env::var(AUTH0_FIELD) and forwards any errors to the caller
    pub fn new_from_env() -> Result<Self, crate::Error> {
        Ok(Self {
            domain: std::env::var(ENV_KEY_AUTH0_DOMAIN)?,
            client_id: std::env::var(ENV_KEY_AUTH0_CLIENT_ID)?,
            client_secret: std::env::var(ENV_KEY_AUTH0_CLIENT_SECRET)?,
            audience: std::env::var(ENV_KEY_AUTH0_AUDIENCE)?,
        })
    }
    pub fn get_jwks_url(&self) -> String {
        format!("https://{}/{AUTH0_JWKS_DISCOVERY_ENDPOINT}", self.domain)
    }
    #[cfg(all(feature = "alcoholic_jwt", feature = "reqwest"))]
    pub async fn get_jwks(&self) -> Result<alcoholic_jwt::JWKS, crate::Error> {
        let url = self.get_jwks_url();
        Ok(reqwest::get(url)
            .await?
            .json::<alcoholic_jwt::JWKS>()
            .await?)
    }
}
