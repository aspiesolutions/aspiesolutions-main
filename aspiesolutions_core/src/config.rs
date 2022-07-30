use serde::Deserialize;

use crate::constants::{
    AUTH0_JWKS_DISCOVERY_ENDPOINT, ENV_KEY_AUTH0_AUDIENCE, ENV_KEY_AUTH0_CLIENT_ID,
    ENV_KEY_AUTH0_CLIENT_SECRET, ENV_KEY_AUTH0_DOMAIN,
};
#[cfg_attr(
    any(test, debug_assertions, feature = "enable_derive_debug"),
    derive(Debug)
)]
#[derive(Deserialize, Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    // the string used to identify this api. used to verfiy that tokens were meant for this api
    pub audience: String,
    // pub jwks_key_id: String,
}

impl Auth0Config {
    /// Constructs a new instance of this struct using std::env::var(AUTH0_FIELD) and panics if not present
    pub fn new_from_env() -> Self {
        Self {
            domain: std::env::var(ENV_KEY_AUTH0_DOMAIN).unwrap_or_else(|var_error| {
                panic!(
                    "Envrionment Variable {} must be defined",
                    ENV_KEY_AUTH0_DOMAIN
                )
            }),
            client_id: std::env::var(ENV_KEY_AUTH0_CLIENT_ID).unwrap_or_else(|_| {
                panic!(
                    "Envrionment Variable {} must be defined",
                    ENV_KEY_AUTH0_DOMAIN
                )
            }),
            client_secret: std::env::var(ENV_KEY_AUTH0_CLIENT_SECRET).unwrap_or_else(|_| {
                panic!(
                    "Envrionment Variable {} must be defined",
                    ENV_KEY_AUTH0_DOMAIN
                )
            }),
            audience: std::env::var(ENV_KEY_AUTH0_AUDIENCE).unwrap_or_else(|_| {
                panic!(
                    "Envrionment Variable {} must be defined",
                    ENV_KEY_AUTH0_AUDIENCE
                )
            }),
        }
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
