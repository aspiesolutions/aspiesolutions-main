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

#[derive(Debug, Clone)]
pub struct Auth0BearerToken {
    token: String,
    claims: Auth0TokenClaims,
}

// pub fn from_option_into_failure<T>(t:T,e:Option<aspiesolutions_core::Error>)-> rocket::request::Outcome<T,>
use aspiesolutions_core::constants::HTTP_HEADER_NAME_AUTHORIZATION;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth0BearerToken {
    type Error = aspiesolutions_core::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config = match req.guard::<&State<Auth0Config>>().await {
            Success(config) => config,
            Failure((status, e)) => {
                return Failure((
                    status,
                    Self::Error::CustomString(String::from(
                        "Failed to gather Auth0Config from managed state",
                    )),
                ))
            }
            Forward(()) => return Forward(()),
        };
        let jwks = match config.get_jwks().await {
            Ok(keys) => keys,
            Err(e) => return Failure((Status::InternalServerError, e)),
        };
        // this request guard assumes that the alg is RS256
        // the signing key id is currently hardcoded and we will need to ask the auth0 management api at some point for the key id
        // this request guard assumes that the first key that matches is the signing key being used.
        // if this behavior is not correct, Auth0 also implements a 'kid' claim that can be used to get the signing key,
        let jwk = match jwks.find(JWKS_KEY_ID) {
            Some(jwk) => jwk,
            None => {
                return request::Outcome::Failure((Status::InternalServerError, Self::Error::None));
            }
        };
        // get the authorization token from headers
        let mut authorization_header_field =
            match req.headers().get_one(HTTP_HEADER_NAME_AUTHORIZATION) {
                Some(h) => h,
                None => {
                    return request::Outcome::Failure((
                        Status::Unauthorized,
                        Self::Error::HttpRequiredHeaderMissing(
                            HTTP_HEADER_NAME_AUTHORIZATION.to_string(),
                        ),
                    ))
                }
            };
        // this fairing assumes that the authorization token is a bearer token prefixed with 'Bearer '
        // but will still work if you leave out the 'Bearer prefix'
        // removes the bearer prefix. equivelent of substring
        if authorization_header_field.starts_with("Bearer") {
            authorization_header_field =
                &authorization_header_field["Bearer".len()..authorization_header_field.len()];
        }
        // removes any whitespace before the token itself
        authorization_header_field = authorization_header_field.trim_start();
        // now we need to verify the token and retrieve its claims.
        let validations = vec![
            alcoholic_jwt::Validation::NotExpired,
            alcoholic_jwt::Validation::SubjectPresent,
            alcoholic_jwt::Validation::Audience(config.audience.clone()),
        ];

        let valid_jwt = match alcoholic_jwt::validate(authorization_header_field, jwk, validations)
        {
            Ok(jwt) => jwt,
            Err(e) => return request::Outcome::Failure((Status::InternalServerError, e.into())),
        };
        let claims = match serde_json::from_value::<Auth0TokenClaims>(valid_jwt.claims) {
            Ok(claims) => claims,
            Err(e) => return request::Outcome::Failure((Status::InternalServerError, e.into())),
        };
        request::Outcome::Success(Self {
            token: authorization_header_field.to_string(),
            claims,
        })
    }
}
