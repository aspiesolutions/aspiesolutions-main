
use async_trait::async_trait;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket_db_pools::Database;
use rocket_db_pools::{rocket::figment::Figment, Config};

#[derive(Debug)]
pub struct RocketDbPool {
    pub conn: sea_orm::DatabaseConnection,
}
#[derive(Deserialize)]
pub struct Auth0Config {

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
pub struct BearerToken(String);


use rocket::request::{self, Request, FromRequest};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
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
