use juniper::{graphql_interface};
use crate::address::Address;
use crate::session::Session;
use crate::user::User;
#[graphql_interface(for=[Address,Session,User])]
pub trait Node {
    fn id(&self) -> Option<NodeValue>;
}