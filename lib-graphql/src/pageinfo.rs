use juniper::GraphQLObject;
/// encodes information for pages ?
#[derive(GraphQLObject)]
pub struct PageInfo {
    has_next_page: bool,
}
