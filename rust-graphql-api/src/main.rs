use aspiesolutions_core::auth0::client::{Auth0Client, Auth0ManagementClient};
use rocket::launch;
pub mod routes;
use rocket_db_pools::Database;
pub use routes::*;
use rust_graphql_api::Auth0Config;
pub mod lib;

#[launch]
async fn rocket() -> _ {
    let _ = dotenv::dotenv().ok();
    let schema = lib_graphql::Schema::new(
        lib_graphql::Query,
        lib_graphql::Mutation,
        lib_graphql::EmptySubscription::default(),
    );
    // create this config from an environtment variable. Dont ever let the server start unless the values are present. use unwrap or expect.
    let auth0_config = Auth0Config::new_from_env();
    let auth0_management_client = Auth0Client::try_new_management_client_async(&auth0_config)
        .await
        .expect("Failed to create Auth0Management client because the request failed")
        .expect("Failed to create the Auth0Management client because the client returned an error");
    rocket::build()
        .attach(lib::Db::init())
        .manage(schema)
        .manage(auth0_config)
        // rocket can only manage one instance of Auth0Client at a time. a wrapper is required in case multiple clients are created
        .manage(Auth0ManagementClient::new(auth0_management_client))
        .mount(
            GRAPHQL_PUBLIC_SEGMENT,
            rocket::routes![
                handle_graphql_post_request,
                handle_graphql_get_request,
                handle_graphql_get_query
            ],
        )
}
