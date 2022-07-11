use juniper::{graphql_interface,ID};
use crate::address::Address;
use crate::session::Session;
#[graphql_interface(for=[Address,Session])]
pub trait Node {
    fn id(&self) -> Option<NodeValue>;
}