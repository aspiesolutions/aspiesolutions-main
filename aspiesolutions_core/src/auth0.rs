use std::time::SystemTime;

use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct TokenClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub scope: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    claims: Option<TokenClaims>,
}

#[derive(Debug, Clone)]
pub struct Auth0BearerToken {
    pub value: String,
    pub claims: TokenClaims,
}
#[derive(Deserialize)]
pub struct Auth0GetManagmentTokenSuccess {
    access_token: String,
    scope: String,
    expires_in: u64,
    token_type: String,
}
impl std::default::Default for Auth0GetManagmentTokenSuccess {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            scope: String::new(),
            expires_in: 0,
            token_type: String::new(),
        }
    }
}
lazy_static::lazy_static! {
    static ref MANAGEMENT_API_TOKEN_RESPONSE: std::sync::Arc<std::sync::Mutex<Option<Auth0GetManagmentTokenSuccess>>> = std::sync::Arc::new(std::sync::Mutex::new(None));
    static ref MANAGMENT_API_TOKEN_CREATED: std::sync::Arc<std::sync::Mutex<Option<SystemTime>>> = std::sync::Arc::new(std:;sync::Mutex::new(None));
}

// gets a managment token
#[cfg(feature = "reqwest")]
pub async fn get_managment_token(
    config: Option<crate::config::Auth0Config>,
    use_cached_response: bool,
) -> Result<Auth0GetManagmentTokenSuccess, crate::Error> {
    use std::collections::HashMap;
    let mut response = Auth0GetManagmentTokenSuccess::default();
    let current_time = std::time::SystemTime::now();
    let cached_management_api_token_response = MANAGEMENT_API_TOKEN_RESPONSE.get_mut().expect("Failed to get mutable reference");
    let MANAGMENT_API_TOKEN_CREATED
    if MANAGEMENT_API_TOKEN_RESPONSE.is_some() && MANAGMENT_API_TOKEN_CREATED.is_some() {
        let cached = MANAGEMENT_API_TOKEN_RESPONSE.unwrap();
        let duration = MANAGMENT_API_TOKEN_CREATED
            .unwrap()
            .elapsed()
            .expect("Failed to get elapsed duration");
        if duration.as_secs() < cached.expires_in {
            response = cached;
        } else {
            MANAGEMENT_API_TOKEN_RESPONSE = None;
            MANAGMENT_API_TOKEN_CREATED = None;
        }
    }
    let config = config.unwrap_or(crate::config::Auth0Config::new_from_env()?);
    let client = reqwest::Client::new();
    let mut form = HashMap::<&str, String>::new();
    form.insert("grant_type", String::from("client_credentials"));
    form.insert("client_id", config.client_id);
    form.insert("client_secret", config.client_secret);
    form.insert("audience", format!("https://{}/api/v2", config.domain));
    response = client
        .post(format!("https://{}/oauth/token", config.domain))
        .form(&form)
        .send()
        .await?
        .json()
        .await?;

    Ok(String::new())
}

// pub fn from_option_into_failure<T>(t:T,e:Option<aspiesolutions_core::Error>)-> rocket::request::Outcome<T,>
use crate::constants::HTTP_HEADER_NAME_AUTHORIZATION;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
#[cfg_attr(feature = "rocket", rocket::async_trait)]
#[cfg(feature = "rocket")]
impl<'r> FromRequest<'r> for Auth0BearerToken {
    type Error = crate::Error;

    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let config = match req
            .guard::<&rocket::State<crate::config::Auth0Config>>()
            .await
        {
            Success(config) => config,
            Failure((status, _)) => {
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
                return request::Outcome::Failure((
                    Status::InternalServerError,
                    Self::Error::CustomString(
                        "Could Not Find a jwk to validate against".to_string(),
                    ),
                ));
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
            value: authorization_header_field.to_string(),
            claims,
        })
    }
}
