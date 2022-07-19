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
        // check for the existance of an environment variable. if found, mount this route at that path
        // used for when external routing is also used
        match std::env::var("GRAPHQL_HANLDER_PUBLIC_SEGMENT") {
            Ok(s)=>{
                s
            }
            _=>{
                routes::GRAPHQL_PUBLIC_SEGMENT.to_string()
            }
        },
        rocket::routes![
            handle_graphql_post_request,
            handle_graphql_get_request,
            handle_graphql_get_query
        ],
    )
}
