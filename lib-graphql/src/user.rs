use juniper::ID;
use juniper::GraphQLObject;
use crate::node::NodeValue;
#[derive(GraphQLObject,Clone)]
pub struct User {
    id: ID
}
// allow calling into to convert from the entity to the graphql user type
impl std::convert::From<entity::user::Model> for User {
    fn from(entity: entity::user::Model) -> Self {
        Self { id: ID::new(entity.id().to_string()) }
    }
}
impl crate::node::Node for User {
    fn id(&self) -> Option<NodeValue> {
        println!("impl node for user!");
        Some(NodeValue::User(self.to_owned()))
    }
}

impl User {
    /// converts an Option<Model> into an Option<User>. implementing std:;convert::from is not allowed in this case
    pub fn map_model_opt(opt_model:Option<entity::user::Model>) -> Option<User> {
        match opt_model {
            Some(model) => Some(model.into()),
            None=>None
        }
    }
}