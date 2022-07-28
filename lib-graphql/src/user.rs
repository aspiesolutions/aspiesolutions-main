use crate::node::NodeValue;
use juniper::GraphQLObject;
use juniper::ID;
#[derive(GraphQLObject, Clone)]
pub struct User {
    pub id: ID,
    pub idp_id: Option<String>,
}

// allow calling into to convert from the entity to the graphql user type
impl std::convert::From<entity::user::Model> for User {
    fn from(entity: entity::user::Model) -> Self {
        Self {
            id: ID::new(entity.id().to_string()),
            idp_id: entity.idp_id,
        }
    }
}
impl crate::node::Node for User {
    fn id(&self) -> Option<NodeValue> {
        println!("impl node for user!");
        Some(NodeValue::User(self.to_owned()))
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUserInput {
    // the provider id must be known at this point
    pub idp_id: String,
}

impl User {
    /// converts an Option<Model> into an Option<User>. implementing std:;convert::from is not allowed in this case
    pub fn map_model_opt(opt_model: Option<entity::user::Model>) -> Option<User> {
        match opt_model {
            Some(model) => Some(model.into()),
            None => None,
        }
    }
}

impl aspiesolutions_core::StructNameSnakeCase for User {
    fn struct_name_snake_case() -> &'static str {
        "user"
    }
}
