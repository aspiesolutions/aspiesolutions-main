pub mod auth0;
pub mod config;
pub mod constants;
pub mod permissions;

// the global type for all identifers
pub type Id = uuid::Uuid;

pub struct VarError(std::env::VarError);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    VarError(#[from] std::env::VarError),
    #[cfg_attr(feature = "reqwest", error("{0}"))]
    #[cfg(feature = "reqwest")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg_attr(feature = "serde_json", error("{0}"))]
    #[cfg(feature = "serde_json")]
    SerdeJsonError(#[from] serde_json::Error),
    #[cfg_attr(feature = "alcoholic_jwt", error("{0}"))]
    #[cfg(feature = "alcoholic_jwt")]
    AlcoholicJwtValidationError(#[from] alcoholic_jwt::ValidationError),
    #[cfg(feature = "rocket")]
    #[cfg_attr(feature = "rocket", error("Missing Required Header '{0}' in Request"))]
    HttpRequiredHeaderMissing(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("{0}")]
    CustomString(String),
    #[error("None")]
    None,
}
impl std::default::Default for Error {
    fn default() -> Self {
        Self::None
    }
}
impl std::convert::From<()> for Error {
    fn from(_: ()) -> Self {
        Self::None
    }
}
// retrieves the name of the current struct at runtime. easier than creating a derive macro
pub trait StructNameSnakeCase {
    fn struct_name_snake_case() -> &'static str;
}
