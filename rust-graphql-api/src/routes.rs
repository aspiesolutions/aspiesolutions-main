#[rocket::post("/graphql")]
pub async fn handle_graphql_post_request() -> &'static str  {
    "hello world"
}
#[rocket::get("/graphql")]
pub async fn handle_graphql_get_request() -> &'static str {
    "hello world"
}
#[rocket::get("/graphql?<query>")]
pub async fn handle_graphql_get_query<'a>(query:&str)-> &str {
    query
}