use sea_orm::prelude::*;
pub struct Model {
    id: Uuid,
    object_id: Uuid,
    type_name: String,
    owner_id: Uuid,
    group_id: Uuid,
}
