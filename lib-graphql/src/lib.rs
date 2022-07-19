use address::Address;
use juniper::GraphQLObject;
use juniper::graphql_interface;
use juniper::graphql_object;
use session::SessionConnection;
pub use juniper::EmptySubscription;
use juniper::ID;

pub mod session;
pub mod pageinfo;
pub mod node;
pub mod address;

use node::{Node,NodeValue};
// this module contains our graphql api
pub struct Context;
impl juniper::Context for Context {}
pub struct Query;
#[graphql_object(context=Context)]
impl Query {
    pub fn node(id: ID) -> NodeValue {
        NodeValue::Address(Address{id})
    }
}



#[derive(GraphQLObject)]
pub struct User {
    id: ID,
    name: String,
    sessions: SessionConnection
}




pub struct Mutation;
#[graphql_object(context=Context)]
impl Mutation {
    pub fn noop() -> Option<String> {
        None
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
pub use juniper::execute;
pub use juniper::execute_sync;

