use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
}