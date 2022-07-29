use lib_graphql::{AuthContext, Context, Schema};
use rocket::State;
use rust_graphql_api::Auth0BearerToken;

pub const GRAPHQL_PUBLIC_SEGMENT: &str = "/api/graphql";
// we have to mount these paths as '/' and have the runners of this program choose whether to mount thim as '/' or as '/graphql'
// for instances where we have external routing defined

// creates a user entry in the database based only on what can be verified from an access token
// used as an escape hatch to create a new user when a token is first used
// does not check if the user exists already
#[rocket::post("/api/create-user-from-token")]
pub async fn create_user_from_token(
    token: Auth0BearerToken,
    db: rocket_db_pools::Connection<crate::lib::Db>,
) -> Result<rocket::http::Status, aspiesolutions_core::Error> {
    use sea_orm::prelude::*;
    let conn = db.into_inner();
    let active_model = entity::user::ActiveModel {
        idp_id: sea_orm::ActiveValue::Set(token.claims.sub),
        email: sea_orm::ActiveValue::Set(None),
        email_verified: sea_orm::ActiveValue::Set(None),
        image: sea_orm::ActiveValue::Set(None),
        name: sea_orm::ActiveValue::Set(None),
        // object_id: sea_orm::ActiveValue::NotSet,
        id: sea_orm::ActiveValue::NotSet,
    };
    active_model.insert(&conn).await?;
    Ok(rocket::http::Status::Created)
}

#[rocket::post("/", format = "json", data = "<body>")]
pub async fn handle_graphql_post_request(
    token: Auth0BearerToken,
    db: rocket_db_pools::Connection<crate::lib::Db>,
    schema: &State<Schema>,
    body: juniper_rocket::GraphQLRequest,
) -> Result<juniper_rocket::GraphQLResponse, aspiesolutions_core::Error> {
    use sea_orm::prelude::*;

    println!("got bearer token {:#?}", token);
    let conn = db.into_inner();
    let user = match entity::user::Entity::find()
        .filter(entity::user::Column::IdpId.eq(token.claims.sub.as_str()))
        .one(&conn)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(aspiesolutions_core::Error::UserNotFoundError(
                "Could not find a user reference with the given subject".to_string(),
            ))
        }
    };

    let context = Context {
        conn,
        auth: Some(AuthContext {
            // token: Some(token.value),
            claims: token.claims,
            // user,
        }),
    };
    Ok(body.execute(&*schema, &context).await)
}
#[rocket::get("/")]
pub async fn handle_graphql_get_request(schema: &State<Schema>) -> String {
    schema.as_schema_language()
}
#[rocket::get("/?<query>")]
pub async fn handle_graphql_get_query(
    token: Auth0BearerToken,
    db: rocket_db_pools::Connection<crate::lib::Db>,
    schema: &State<Schema>,
    query: juniper_rocket::GraphQLRequest,
) -> Result<juniper_rocket::GraphQLResponse, aspiesolutions_core::Error> {
    use sea_orm::prelude::*;
    // we must call into inner here to get the underlying databsae connection
    println!("got bearer token {:#?}", token);
    let conn = db.into_inner();

    let context = Context {
        conn,
        auth: Some(AuthContext {
            claims: token.claims,
        }),
    };
    Ok(query.execute(&*schema, &context).await)
}
#[cfg(test)]
pub mod tests {
    // use std::collections::HashMap;

    use rocket::{futures::TryFutureExt, http::ContentType};

    #[tokio::test]
    pub async fn test_post_graphql() {
        let rocket = crate::rocket().await;
        let client = rocket::local::asynchronous::Client::tracked(rocket)
            .unwrap_or_else(|error| panic!("Failed to build rocket instance {error}"))
            .await;
        // let mut body = HashMap::<&str, &str>::new();
        let test_query = r#"
        {"source":"
            query testQuery {
                accessCode(id:\"abcd\") {
                    user: {
                        id
                    }
                    errors
                }
            }
        "}"#;
        // body.insert("source", test_query);
        let response = client
            .post("/api/graphql")
            .header(ContentType::JSON)
            .body(test_query)
            .dispatch()
            .await;
        // let status = response.status();
        let response_text = response.into_string().await;
        println!("{:#?}", response_text)
    }
}
