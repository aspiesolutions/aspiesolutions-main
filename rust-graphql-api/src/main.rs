use rocket::{launch, State};
pub mod routes;
use rocket_db_pools::Database;
pub use routes::*;
pub mod lib;

#[launch]
fn rocket() -> _ {
    let schema = lib_graphql::Schema::new(
        lib_graphql::Query,
        lib_graphql::Mutation,
        lib_graphql::EmptySubscription::default(),
    );
    rocket::build()
    .attach(lib::Db::init()).manage(schema).mount(
        "/",
        rocket::routes![
            handle_graphql_post_request,
            handle_graphql_get_request,
            handle_graphql_get_query
        ],
    )
}
