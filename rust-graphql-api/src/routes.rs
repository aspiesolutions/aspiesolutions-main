use lib_graphql::{Schema, Context};
use rocket::State;

#[rocket::post("/graphql",format="json",data="<body>")]
pub async fn handle_graphql_post_request(schema: &State<Schema>,body: juniper_rocket::GraphQLRequest) -> juniper_rocket::GraphQLResponse  {
    let context = Context {};
    body.execute(&*schema, &context).await

}
#[rocket::get("/graphql")]
pub async fn handle_graphql_get_request<'a>(schema: &State<Schema>) -> String {
    schema.as_schema_language()
}
#[rocket::get("/graphql?<query>")]
pub async fn handle_graphql_get_query<'a>(schema: &State<Schema>,query:juniper_rocket::GraphQLRequest)-> juniper_rocket::GraphQLResponse {
    let context = Context {};
    query.execute(&*schema, &context).await
}