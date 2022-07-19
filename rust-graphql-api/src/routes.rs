use lib_graphql::{Schema, Context};
use rocket::State;

pub const GRAPHQL_PUBLIC_SEGMENT:&'static str ="/graphql";
// we have to mount these paths as '/' and have the runners of this program choose whether to mount thim as '/' or as '/graphql'
// for instances where we have external routing defined
#[rocket::post("/",format="json",data="<body>")]
pub async fn handle_graphql_post_request(schema: &State<Schema>,body: juniper_rocket::GraphQLRequest) -> juniper_rocket::GraphQLResponse  {
    let context = Context {};
    body.execute(&*schema, &context).await

}
#[rocket::get("/")]
pub async fn handle_graphql_get_request<'a>(schema: &State<Schema>) -> String {
    schema.as_schema_language()
}
#[rocket::get("/?<query>")]
pub async fn handle_graphql_get_query<'a>(schema: &State<Schema>,query:juniper_rocket::GraphQLRequest)-> juniper_rocket::GraphQLResponse {
    let context = Context {};
    query.execute(&*schema, &context).await
}