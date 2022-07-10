use rocket::{launch};
pub mod routes;
pub use routes::*;

#[launch]
fn rocket()->_ {
    rocket::build().mount("/",rocket::routes![handle_graphql_post_request,handle_graphql_get_request,handle_graphql_get_query])
}