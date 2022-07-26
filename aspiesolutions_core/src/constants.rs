use const_format::concatcp;


/// the string prefix for all auth0 environment variables
pub const AUTH0_ENV_PREFIX: &'static str = "AUTH0";
const ENV_SEPERATOR:&'static str = "_";
/// the string suffix for auth0 domain
pub const ENV_KEY_AUTH0_DOMAIN: &'static str = concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "DOMAIN");
/// the string suffix used to fetch auth0 client id
pub const ENV_KEY_AUTH0_CLIENT_ID:&'static str = concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "CLIENT_ID");
/// the string suffix used to fetch auth0 client secret
pub const ENV_KEY_AUTH0_CLIENT_SECRET:&'static str = concatcp!(AUTH0_ENV_PREFIX,ENV_SEPERATOR,"CLIENT_SECRET");
pub const ENV_KEY_DATABASE_URL: &'static str = "DATABASE_URL";

/// the path segment used to fetch JSON Web Keysets. part of the OpenID Connect specification
pub const AUTH0_JWKS_DISCOVERY_ENDPOINT: &'static str = ".well-known/jwks.json";