pub mod auth0;
pub mod config;
pub mod constants;

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
// #[cfg(feature="rocket")]
// impl std::convert::Into<Error> for (rocket::request::Status,()) {
//     fn into(outcome:(rocket::http::Status,Error)) -> Self {
//         (outcome.0,())

//     }
// }
