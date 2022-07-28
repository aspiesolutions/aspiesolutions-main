use const_format::concatcp;
use reqwest::{header, Response, StatusCode};
use serde::Deserialize;

const GRANT_TYPE_CLIENT_CREDENTIALS: &'static str = "client_credentials";
const HEADER_NAME_AUTHENTICATION: &'static str = "authorization";
const HEADER_NAME_CONTENT_TYPE: &'static str = "content-typ";
const FORM_DATA_KEY_GRANT_TYPE: &'static str = "grant_type";
const FORM_DATA_KEY_CLIENT_ID: &'static str = "client_id";
const FORM_DATA_KEY_CLIENT_SECRET: &'static str = "client_secret";
const FORM_DATA_KEY_AUDIENCE: &'static str = "audience";

const SCOPE_SEPERATOR: &'static str = ":";
const SCOPE_ACTION_READ: &'static str = "read";
const SCOPE_SUBJECT_SIGNING_KEYS: &'static str = "signing_keys";
const SCOPE_READ_SIGNING_KEYS: &'static str = concatcp!(
    SCOPE_ACTION_READ,
    SCOPE_SEPERATOR,
    SCOPE_SUBJECT_SIGNING_KEYS
);

#[derive(Debug, Clone)]
pub struct Auth0Client {
    auth0_config: std::sync::Arc<crate::config::Auth0Config>,
    access_token: String,
    scope: String,
    client: std::sync::Arc<reqwest::Client>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Auth0GetManagmentTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Auth0ErrorResponse {
    error: String,
    error_description: String,
}
#[derive(Clone, Debug, thiserror::Error)]
pub enum Auth0ClientError {
    #[error("Unauthorized: {0:#?}")]
    Unauthorized(Auth0ErrorResponse),
    #[error("Unauthorized: Client does not have permission to use scope(s) '{0}'")]
    RequiredScopeMissing(String),
}
#[derive(Clone, Debug)]
pub enum Auth0GetManagmentTokenError {
    Unauthorized(Auth0ErrorResponse),
}
#[derive(Deserialize, Clone, Debug)]
pub struct Auth0ApplicationSigningKey {
    kid: String,
    cert: String,
    pkcs7: String,
    current: Option<bool>,
    next: Option<bool>,
    previous: Option<bool>,
    current_since: Option<String>,
    current_until: Option<String>,
    fingerprint: String,
    // revoked: bool,
    // revoked_at: String,
}
impl Auth0Client {
    fn create_reqwest_client(access_token:&str) -> Result<reqwest::Client, reqwest::Error> {
        // create the header map
        let mut headers = reqwest::header::HeaderMap::new();
        // the access token is a raw string without a prefix. it has to be prefixed with "Bearer " to be accepted by the authentication server
        let bearer_token_string = format!("Bearer {}", access_token);
        // create the header value to be placed in the map
        let mut access_token_header_value =
        reqwest::header::HeaderValue::from_str(&bearer_token_string)
            .expect("Failed to build authentication header value for Auth0Client");
        
   
    // the token is sensitive information
    access_token_header_value.set_sensitive(true);
    headers.insert(HEADER_NAME_AUTHENTICATION, access_token_header_value);
    Ok(reqwest::ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(60_u64))
    .default_headers(headers)
    .https_only(true)
    .build()
    .expect("Failed to build reqwest::Client for Auth0Client"))
    }
    pub async fn try_new_management_client_async(
        config: &crate::config::Auth0Config,
    ) -> Result<Result<Self, Auth0ClientError>, reqwest::Error> {
        // get a management token before creating the client
        let management_token_response = match Self::get_managment_token_async(config).await? {
            Ok(response) => response,
            Err(e) => return Ok(Err(e)),
        };
        let client = Self::create_reqwest_client(&management_token_response.access_token)?;

        // the teturn type is Result<Result<_,_>,_> to signify that request could fail
        Ok(Ok(Self {
            auth0_config: std::sync::Arc::new(config.clone()),
            client: std::sync::Arc::new(client),
            access_token: management_token_response.access_token,
            scope: management_token_response.scope,
        }))
    }
    pub async fn get_managment_token_async(
        config: &crate::config::Auth0Config,
    ) -> Result<Result<Auth0GetManagmentTokenResponse, Auth0ClientError>, reqwest::Error> {
        let audience = format!("https://{}/api/v2/", config.domain);
        let mut form = std::collections::HashMap::<&str, &str>::new();
        form.insert(FORM_DATA_KEY_GRANT_TYPE, GRANT_TYPE_CLIENT_CREDENTIALS);
        form.insert(FORM_DATA_KEY_CLIENT_ID, &config.client_id);
        form.insert(FORM_DATA_KEY_CLIENT_SECRET, &config.client_secret);
        form.insert(FORM_DATA_KEY_AUDIENCE, &audience);
        let response = reqwest::Client::new()
            .post(&format!("https://{}/oauth/token", config.domain))
            .form(&form)
            .send()
            .await?;
        let status = response.status();
        if status == StatusCode::OK {
            return Ok(Ok(response
                .json::<Auth0GetManagmentTokenResponse>()
                .await?));
        }
        let error_response_body: Auth0ErrorResponse = response.json().await?;
        Ok(Err(Auth0ClientError::Unauthorized(error_response_body)))
    }
    /// use this function when the token expires. gets a new managment token
    async fn refresh_managment_client(
        &mut self,
    ) -> Result<Result<(), Auth0ClientError>, reqwest::Error> {
        let response = match Self::get_managment_token_async(&self.auth0_config).await? {
            Ok(response) => response,
            Err(e) => return Ok(Err(e)),
        };
        let client = Self::create_reqwest_client(&response.access_token)?;
        self.access_token = response.access_token;
        self.scope = response.scope;
        self.client = std::sync::Arc::new(client);

        Ok(Ok(()))
    }
    pub async fn management_get_all_application_signing_keys(
        &self,
    ) -> Result<Result<Vec<Auth0ApplicationSigningKey>, Auth0ClientError>, reqwest::Error> {
        if !self.scope.contains(SCOPE_READ_SIGNING_KEYS) {
            log::debug!(
                "self.scope does not contain scope '{}'. request will fail",
                SCOPE_SUBJECT_SIGNING_KEYS
            );
            return Ok(Err(Auth0ClientError::RequiredScopeMissing(
                SCOPE_READ_SIGNING_KEYS.to_string(),
            )));
        }
        let response = self
            .client
            .get(&format!(
                "https://{}/api/v2/keys/signing",
                &self.auth0_config.domain
            ))
            .send()
            .await?;
        let status = response.status();
        match status {
            StatusCode::OK => {
                let body = response
                .json::<Vec<Auth0ApplicationSigningKey>>()
                .await?;
                println!("{:#?}",body);
                return Ok(Ok(body));
            }
            StatusCode::UNAUTHORIZED => {
                let body = response.text().await?;
                println!("get signing_keys_response {body:#?}");
                todo!("handle unauthorized case")
            }
            _ => {
                unimplemented!("unhandled response status code {status} in function managment_api_get_all_application_sigining_keys")
            }
        }
    }
}
#[cfg(test)]
pub mod tests {
    use super::{Auth0Client, Auth0GetManagmentTokenResponse};
    use crate::config::Auth0Config;
    fn get_config() -> Auth0Config {
        dotenv::dotenv().ok();
        Auth0Config::new_from_env()
    }
    async fn get_management_token() -> Auth0GetManagmentTokenResponse {
        Auth0Client::get_managment_token_async(&get_config())
            .await
            .expect("Auth0Client: request failed!")
            .expect("the client returned an error")
    }
    async fn create_managment_client() -> Auth0Client {
        Auth0Client::try_new_management_client_async(&get_config())
            .await
            .expect("Auth0Client create failed because the request to the external server failed")
            .expect("The client returned an error")
    }
    #[tokio::test]
    pub async fn test_get_management_token() {
        let _response = get_management_token().await;
    }
    #[tokio::test]
    pub async fn test_create_managment_client() {
        let _client = create_managment_client().await;
    }
    #[tokio::test]
    pub async fn test_management_get_all_siging_keys() {
        let client = create_managment_client().await;
        let _response = client
            .management_get_all_application_signing_keys()
            .await
            .expect("Auth0Client get_all_application_signing_keys() request failed!")
            .expect("the client returned an error. Make sure that you have correctly configured your domain and that you have scopes neccessary to perform this action");
    }
}
