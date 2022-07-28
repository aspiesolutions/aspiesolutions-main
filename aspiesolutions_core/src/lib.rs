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
    RocketHttpRequiredHeaderMissing(String),
    #[cfg(feature = "rocket")]
    #[cfg_attr(feature = "rocket", error("Missing managed state in request"))]
    RocketMissingState,
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Error: {0}")]
    CustomString(String),
    #[error("Failed to convert from str: {0}")]
    FromStrError(String),
    #[error("Error: {0}")]
    UuidError(#[from] uuid::Error),
    #[cfg_attr(feature = "sea-orm", error("{0}"))]
    #[cfg(feature = "sea-orm")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("User not found {0}")]
    UserNotFoundError(String),
}
pub trait StructNameSnakeCase {
    fn struct_name_snake_case() -> &'static str;
}
#[cfg(feature = "rocket")]
impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        use rocket::http::Status;
        log::error!("{}", &self);
        match self {
            Error::Unauthorized(message) => {
                log::error!("Unauthorized {}", message);
                Err(Status::Unauthorized)
            }
            Error::RocketHttpRequiredHeaderMissing(_) | Error::AlcoholicJwtValidationError(_) => {
                Err(Status::BadRequest)
            }
            Error::UserNotFoundError(_) => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
