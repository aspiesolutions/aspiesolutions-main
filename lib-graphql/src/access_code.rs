use juniper::{GraphQLObject, ID};

#[derive(GraphQLObject)]
pub struct AccessCode {
    id: ID,
}
