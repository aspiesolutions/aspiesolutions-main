use juniper::{GraphQLObject};
/// encodes information for pages ?
#[derive(GraphQLObject)]
pub struct PageInfo {
    hasNextPage: bool
}