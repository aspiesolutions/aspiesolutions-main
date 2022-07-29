// use sea_orm::prelude::*;
// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
// #[sea_orm(table_name = "group")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     id: Uuid,
//     name: String,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl Related<super::user::Entity> for Entity {
//     fn to() -> RelationDef {
//         super::user_group::Relation::User.def()
//     }
//     fn via() -> Option<RelationDef> {
//         Some(super::user_group::Relation::Group.def().rev())
//     }
// }
// impl ActiveModelBehavior for ActiveModel {}
