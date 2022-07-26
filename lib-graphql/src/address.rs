use crate::node::{Node, NodeValue};
use juniper::{GraphQLObject, ID};
#[derive(GraphQLObject, Clone)]
#[graphql(impl=[NodeValue])]
pub struct Address {
    pub id: ID,
}
impl Node for Address {
    fn id(&self) -> Option<NodeValue> {
        Some(NodeValue::Address(self.to_owned()))
    }
}
