use const_format::concatcp;

/// the string prefix for all auth0 environment variables
pub const AUTH0_ENV_PREFIX: & str = "AUTH0";
const ENV_SEPERATOR: & str = "_";
/// the string suffix for auth0 domain
pub const ENV_KEY_AUTH0_DOMAIN: & str = concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "DOMAIN");
/// the string suffix used to fetch auth0 client id
pub const ENV_KEY_AUTH0_CLIENT_ID: & str =
    concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "CLIENT_ID");
/// the string suffix used to fetch auth0 client secret
pub const ENV_KEY_AUTH0_CLIENT_SECRET: & str =
    concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "CLIENT_SECRET");
// the string used to fetch auth0 audience
pub const ENV_KEY_AUTH0_AUDIENCE: & str =
    concatcp!(AUTH0_ENV_PREFIX, ENV_SEPERATOR, "AUDIENCE");
pub const ENV_KEY_DATABASE_URL: & str = "DATABASE_URL";

/// the path segment used to fetch JSON Web Keysets. part of the OpenID Connect specification
pub const AUTH0_JWKS_DISCOVERY_ENDPOINT: & str = ".well-known/jwks.json";

pub const HTTP_HEADER_NAME_AUTHORIZATION: & str = "authorization";
