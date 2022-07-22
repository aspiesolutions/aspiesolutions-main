use lib_graphql::{Schema, Context};
use rocket::State;

pub const GRAPHQL_PUBLIC_SEGMENT:&'static str ="/api/graphql";
// we have to mount these paths as '/' and have the runners of this program choose whether to mount thim as '/' or as '/graphql'
// for instances where we have external routing defined
#[rocket::post("/",format="json",data="<body>")]
pub async fn handle_graphql_post_request(db: rocket_db_pools::Connection<crate::lib::Db>,schema: &State<Schema>,body: juniper_rocket::GraphQLRequest, ) -> juniper_rocket::GraphQLResponse  {
    let conn = db.into_inner();
    let context = Context {conn};
    body.execute(&*schema, &context).await

}
#[rocket::get("/")]
pub async fn handle_graphql_get_request(schema: &State<Schema>) -> String {
    schema.as_schema_language()
}
#[rocket::get("/?<query>")]
pub async fn handle_graphql_get_query(db: rocket_db_pools::Connection<crate::lib::Db>,schema: &State<Schema>,query:juniper_rocket::GraphQLRequest)-> juniper_rocket::GraphQLResponse {
    // we must call into inner here to get the underlying databsae connection
    let conn = db.into_inner();
    let context = Context {conn};
    query.execute(&*schema, &context).await
}