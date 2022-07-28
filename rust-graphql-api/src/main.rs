use rocket::launch;
pub mod routes;
use rocket_db_pools::Database;
pub use routes::*;
use rust_graphql_api::Auth0Config;
pub mod lib;

#[launch]
fn rocket() -> _ {
    let schema = lib_graphql::Schema::new(
        lib_graphql::Query,
        lib_graphql::Mutation,
        lib_graphql::EmptySubscription::default(),
    );
    // create this config from an environtment variable. Dont ever let the server start unless the values are present. use unwrap or expect.
    let auth0_config = Auth0Config::new_from_env();
    rocket::build()
        .attach(lib::Db::init())
        .manage(schema)
        .manage(auth0_config)
        .mount(
            GRAPHQL_PUBLIC_SEGMENT,
            rocket::routes![
                handle_graphql_post_request,
                handle_graphql_get_request,
                handle_graphql_get_query
            ],
        )
}
