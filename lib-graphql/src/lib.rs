use juniper::{EmptySubscription, ID};
use neon::context;

// this module contains our graphql api
pub struct Query;
#[graphql_object]
impl Query {
    pub fn node(id:ID) -> Option<String> {
        Some("hello world".to_string())
    }
}


pub struct Mutation;
#[graphql_object]
impl Mutation {
    pub fn noop() -> Option<String> {
        None
    }
}

pub struct Context;
impl juniper::Context for Context{}



pub type Schema = juniper::RootNode<'static, Query, Mutation,EmptySubscription<()>>;

lazy_static::lazy_static! {
    pub static ref SCHEMA:Schema = Schema::new(Query, Mutation, EmptySubscription::<()>::default());
}
