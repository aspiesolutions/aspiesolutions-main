
use async_trait::async_trait;
use rocket::State;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket_db_pools::Database;
use rocket_db_pools::{rocket::figment::Figment, Config};


// for now, we will hard code the jwks endpoint

pub const JWKS_DISCOVERY_ENDPOINT:&'static str = ".well-known/jwks.json";

#[derive(Debug)]
pub struct RocketDbPool {
    pub conn: sea_orm::DatabaseConnection,
}


#[derive(Deserialize,Debug)]
pub struct Auth0Config {
    pub issuer:String,
    pub client_id:String,
    pub client_secret:String,
}
pub const AUTH0_ENV_PREFIX: &'static str = "AUTH0";
impl Auth0Config {
    /// Constructs a new instance of this struct using std::env::var(AUTH0_FIELD) and forwards any errors to the caller
    pub fn new_from_env()-> Result<Self,std::env::VarError> {
        Ok(Self {
            issuer: std::env::var(&format!("{AUTH0_ENV_PREFIX}_ISSUER"))?,
            client_id: std::env::var(&format!("{AUTH0_ENV_PREFIX}_CLIENT_ID"))?,
            client_secret: std::env::var(&format!("{AUTH0_ENV_PREFIX}_CLIENT_SECRET"))?
        })
    }
}

#[async_trait]
impl rocket_db_pools::Pool for RocketDbPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {

        let config = figment.extract::<Config>().unwrap();

        let conn = sea_orm::Database::connect(&config.url).await.unwrap();
        return Ok(RocketDbPool { conn })

    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }
}

#[derive(Database, Debug)]
#[database("default")]
pub struct Db(RocketDbPool);


#[derive(Debug,Clone)]
pub struct Auth0BearerToken(String);


use rocket::request::{self, Request, FromRequest};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth0BearerToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config_outcome = req.guard::<&State<Auth0Config>>().await;
        if config_outcome.is_failure() {
            return request::Outcome::Failure((Status::InternalServerError,()));
        }
        if config_outcome.is_forward() {
            return request::Outcome::Forward(());
        }
        let config = config_outcome.unwrap();
        println!("Got Config! {:#?}",config);
        let authorization_option = req.headers().get_one("authorization");
        if authorization_option.is_none() {
            return request::Outcome::Failure((Status::Unauthorized,()))
        }
        let mut authorization =  authorization_option.unwrap();
        if authorization.starts_with("Bearer") {
            authorization = &authorization["Bearer".len()..authorization.len()];
        }
        authorization = authorization.trim_start();
        request::Outcome::Success(Self(authorization.to_string()))
    }
}
