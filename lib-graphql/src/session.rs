use juniper::{ID,GraphQLObject};
use crate::node::{Node,NodeValue};
use crate::pageinfo::PageInfo;
#[derive(GraphQLObject)]
pub struct SessionConnection {
    edges:Vec<SessionEdge>,
    pageInfo: PageInfo
}
#[derive(GraphQLObject)]
pub struct SessionEdge {
    cursor: String,
    node: Option<Session>
}
#[derive(GraphQLObject,Clone)]
#[graphql(impl=NodeValue)]
pub struct Session {
    id: ID
}

impl Node for Session {
    fn id(&self)-> Option<NodeValue> {
        Some(NodeValue::Session(self.to_owned()))
    }
}