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
