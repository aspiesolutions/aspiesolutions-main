use std::time::SystemTime;

#[cfg(feature = "reqwest")]
pub mod client;
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone, Default)]
pub struct TokenClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub scope: Option<String>,
}
// #[derive(Deserialize, Debug, Clone)]
// pub struct Context {
//     claims: Option<TokenClaims>,
// }

#[derive(Debug, Clone)]
pub struct Auth0BearerToken {
    pub value: String,
    pub claims: TokenClaims,
}
#[derive(Deserialize, Clone, Default)]
#[allow(unused)]
pub struct Auth0GetManagmentTokenSuccess {
    access_token: String,
    scope: String,
    expires_in: u64,
    token_type: String,
}

lazy_static::lazy_static! {
    static ref MANAGEMENT_API_TOKEN_RESPONSE: std::sync::Arc<tokio::sync::Mutex<Option<Auth0GetManagmentTokenSuccess>>> = std::sync::Arc::new(tokio::sync::Mutex::new(None));
    static ref MANAGMENT_API_TOKEN_CREATED: std::sync::Arc<tokio::sync::Mutex<Option<SystemTime>>> = std::sync::Arc::new(tokio::sync::Mutex::new(None));
}

// gets a managment token
#[cfg(feature = "reqwest")]
pub async fn get_managment_token(
    config: &crate::config::Auth0Config,
    // use_cached_response: bool,
) -> Result<Auth0GetManagmentTokenSuccess, crate::Error> {
    use std::collections::HashMap;
    // let current_time = std::time::SystemTime::now();
    let mut cached_management_api_token_response = MANAGEMENT_API_TOKEN_RESPONSE.lock().await;

    let mut cached_management_api_token_created = MANAGMENT_API_TOKEN_CREATED.lock().await;
    if cached_management_api_token_response.is_some()
        && cached_management_api_token_created.is_some()
    {
        let cached = cached_management_api_token_response.as_ref().unwrap();
        let duration = cached_management_api_token_created
            .unwrap()
            .elapsed()
            .expect("Failed to get elapsed duration");
        if duration.as_secs() < cached.expires_in {
            return Ok(cached.clone());
        } else {
            *cached_management_api_token_response = None;
            *cached_management_api_token_created = None;
        }
    }
    let client = reqwest::Client::new();
    let mut form = HashMap::<&str, String>::new();
    form.insert("grant_type", String::from("client_credentials"));
    form.insert("client_id", config.client_id.clone());
    form.insert("client_secret", config.client_secret.clone());
    form.insert("audience", format!("https://{}/api/v2", config.domain));
    Ok(client
        .post(format!("https://{}/oauth/token", config.domain))
        .form(&form)
        .send()
        .await?
        .json()
        .await?)
}
// pub async fn get_client(
//     config: &Auth0Config,
//     access_token: &str,
//     client_id: &str,
// ) -> Result<(), crate::Error> {
//     let client = reqwest::Client::new();
//     let response = client
//         .get(format!(
//             "https://{}/api/v2/clients/{}",
//             config.domain, config.client_id
//         ))
//         .header("authorization", format!("Bearer {}", access_token))
//         .send()
//         .await?;
//     println!("response {response:#?}");
//     Ok(())
// }
// pub fn from_option_into_failure<T>(t:T,e:Option<aspiesolutions_core::Error>)-> rocket::request::Outcome<T,>
use crate::constants::HTTP_HEADER_NAME_AUTHORIZATION;
#[cfg_attr(feature = "rocket", rocket::async_trait)]
#[cfg(feature = "rocket")]
impl<'r> rocket::request::FromRequest<'r> for Auth0BearerToken {
    type Error = crate::Error;

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::http::Status;
        use rocket::outcome::Outcome::*;
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
        let managment_client = match req
            .guard::<&rocket::State<crate::auth0::client::Auth0ManagementClient>>()
            .await
        {
            Success(client) => client,
            Failure((status, _error)) => {
                return Failure((status, crate::Error::RocketMissingState))
            }
            Forward(status) => return Forward(status),
        };
        // we now have a managment api client to be able to fetch the siging keys
        let signing_keys = match managment_client.get_all_application_signing_keys().await {
            Ok(Ok(keys)) => keys,
            Ok(Err(client_error)) => {
                return Failure((
                    rocket::http::Status::InternalServerError,
                    crate::Error::CustomString(client_error.to_string()),
                ))
            }
            Err(reqwest_error) => {
                return Failure((
                    rocket::http::Status::InternalServerError,
                    crate::Error::CustomString(reqwest_error.to_string()),
                ))
            }
        };
        let signing_key = match signing_keys.get_current_key() {
            Some(key) => key,
            None => {
                return Failure((
                    rocket::http::Status::InternalServerError,
                    crate::Error::CustomString(
                        "Could not get signing key as no current key is available".to_string(),
                    ),
                ))
            }
        };
        // I am unsure if the JWT can be verified from the cert and pkcs7 and thumbprint returned from this backchannel communication.
        // all we really cared about is getting the kid of the current token being used.As such, we have to fetch the JWK from the public endpoint

        let jwks = match config.get_jwks().await {
            Ok(keys) => keys,
            Err(e) => return Failure((Status::InternalServerError, e)),
        };
        // this request guard assumes that the alg is RS256
        // the signing key id is retrieved using backchannel communication that determines the active key being used

        let jwk = match jwks.find(signing_key.kid()) {
            Some(jwk) => jwk,
            None => {
                return Failure((
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
                    return Failure((
                        Status::Unauthorized,
                        Self::Error::RocketHttpRequiredHeaderMissing(
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
            Err(e) => return Failure((Status::InternalServerError, e.into())),
        };
        let claims = match serde_json::from_value::<TokenClaims>(valid_jwt.claims) {
            Ok(claims) => claims,
            Err(e) => return Failure((Status::InternalServerError, e.into())),
        };
        Success(Self {
            value: authorization_header_field.to_string(),
            claims,
        })
    }
}
#[cfg(test)]
pub mod tests {}
