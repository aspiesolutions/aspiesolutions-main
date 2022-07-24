use alcoholic_jwt::Validation;
use anyhow::Context;
use async_trait::async_trait;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_db_pools::Database;
use rocket_db_pools::{rocket::figment::Figment, Config};

// for now, we will hard code the jwks endpoint

pub const JWKS_DISCOVERY_ENDPOINT: &'static str = ".well-known/jwks.json";
/// the id of the public signing key. curerntly hardcoded.
pub const JWKS_KEY_ID: &'static str = "yo4HXbTKFVHwdZ6_MD0CE";
#[derive(Debug)]
pub struct RocketDbPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[derive(Deserialize, Debug)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
}
pub const AUTH0_ENV_PREFIX: &'static str = "AUTH0";
impl Auth0Config {
    fn get_key_with_prefix<'a>(key: &'a str) -> String {
        format!("{AUTH0_ENV_PREFIX}_{}", key)
    }
    fn get_domain_key() -> String {
        Self::get_key_with_prefix("DOMAIN")
    }
    fn get_client_id_key() -> String {
        Self::get_key_with_prefix("CLIENT_ID")
    }
    fn get_client_secret_key() -> String {
        Self::get_key_with_prefix("CLIENT_SECRET")
    }
    /// Constructs a new instance of this struct using std::env::var(AUTH0_FIELD) and forwards any errors to the caller
    pub fn new_from_env() -> Result<Self, anyhow::Error> {
        Ok(Self {
            domain: std::env::var(&Self::get_domain_key())
                .with_context(|| Self::get_domain_key())?,
            client_id: std::env::var(&Self::get_client_id_key())
                .with_context(|| Self::get_client_id_key())?,
            client_secret: std::env::var(&Self::get_client_secret_key())
                .with_context(|| Self::get_client_secret_key())?,
        })
    }
    pub fn get_jwks_url(&self) -> String {
        format!("https://{}/{JWKS_DISCOVERY_ENDPOINT}", self.domain)
    }
    pub async fn get_jwks(&self) -> Result<alcoholic_jwt::JWKS, anyhow::Error> {
        let url = self.get_jwks_url();
        Ok(reqwest::get(url)
            .await?
            .json::<alcoholic_jwt::JWKS>()
            .await?)
    }
}

#[async_trait]
impl rocket_db_pools::Pool for RocketDbPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();

        println!("{}",config.url);
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

#[derive(Debug, Clone)]
pub struct Auth0BearerToken(String);

#[derive(Deserialize, Debug)]
pub struct Auth0Jwt {
    iss:String
}

use rocket::request::{self, FromRequest, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth0BearerToken {
    type Error = Option<anyhow::Error>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config_outcome = req.guard::<&State<Auth0Config>>().await;
        if config_outcome.is_failure() {
            return request::Outcome::Failure((Status::InternalServerError, None));
        }
        if config_outcome.is_forward() {
            return request::Outcome::Forward(());
        }
        let config = config_outcome.unwrap();
        let jwks_result = config.get_jwks().await;
        if jwks_result.is_err() {
            return request::Outcome::Failure((
                Status::InternalServerError,
                Some(jwks_result.unwrap_err()),
            ));
        }
        let jwks = jwks_result.unwrap();
        // this request guard assumes that the alg is RS256
        // the signing key id is currently hardcoded and we will need to ask the auth0 management api at some point for the key id
        // this request guard assumes that the first key that matches is the signing key being used.
        // if this behavior is not correct, Auth0 also implements a 'kid' claim that can be used to get the signing key,
        let jwk_find_option = jwks.find(JWKS_KEY_ID);
        if jwk_find_option.is_none() {
            return request::Outcome::Failure((Status::InternalServerError, None));
        }
        let jwk = jwk_find_option.unwrap();

        // get the authorization token from headers
        let authorization_option = req.headers().get_one("authorization");
        // convert the none case into a failure outcome
        if authorization_option.is_none() {
            return request::Outcome::Failure((Status::Unauthorized, None));
        }
        // this fairing assumes that the authorization token is a bearer token prefixed with 'Bearer '
        // but will still work if you leave out the 'Bearer prefix'
        let mut token = authorization_option.unwrap();
        // removes the bearer prefix
        if token.starts_with("Bearer") {
            token = &token["Bearer".len()..token.len()];
        }
        // removes any whitespace before the token itself
        token = token.trim_start();
        println!("{token}");
        // now we need to verify the token and retrieve its claims.
        let validations = Vec::<Validation>::new();

        let valid_jwt = match alcoholic_jwt::validate(token, jwk, validations) {
            Ok(jwt) => jwt,
            Err(e) => {
                return request::Outcome::Failure((Status::InternalServerError, Some(e.into())))
            }
        };
        let claims = match serde_json::from_value::<Auth0Jwt>(valid_jwt.claims) {
            Ok(claims)=>claims,
            Err(e)=> return request::Outcome::Failure((Status::InternalServerError,Some(e.into())))
        };

        println!("valid jwt {:#?}", claims);
        request::Outcome::Success(Self(token.to_string()))
    }
}
