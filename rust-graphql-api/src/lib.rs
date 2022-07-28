use alcoholic_jwt::Validation;
use anyhow::Context;
use async_trait::async_trait;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_db_pools::Database;
use rocket_db_pools::{rocket::figment::Figment, Config};

// for now, we will hard code the jwks endpoint
pub use aspiesolutions_core::auth0::TokenClaims as Auth0TokenClaims;
pub use aspiesolutions_core::constants::{
    AUTH0_ENV_PREFIX, AUTH0_JWKS_DISCOVERY_ENDPOINT as JWKS_DISCOVERY_ENDPOINT,
    ENV_KEY_AUTH0_CLIENT_ID, ENV_KEY_AUTH0_CLIENT_SECRET, ENV_KEY_AUTH0_DOMAIN,
};
/// the id of the public signing key. curerntly hardcoded.
pub const JWKS_KEY_ID: &'static str = "yo4HXbTKFVHwdZ6_MD0CE";
#[derive(Debug)]
pub struct RocketDbPool {
    pub conn: sea_orm::DatabaseConnection,
}

pub use aspiesolutions_core::config::Auth0Config;

#[async_trait]
impl rocket_db_pools::Pool for RocketDbPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let conn = sea_orm::Database::connect(&config.url).await.unwrap();
        return Ok(RocketDbPool { conn });
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }
}

#[derive(Database, Debug)]
#[database("default")]
pub struct Db(RocketDbPool);

pub use aspiesolutions_core::auth0::Auth0BearerToken;
