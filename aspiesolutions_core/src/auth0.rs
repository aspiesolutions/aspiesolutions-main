use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct TokenClaims {
    iss: String,
    sub: String,
    aud: String,
    scope: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Context {
    claims: Option<TokenClaims>,
}
