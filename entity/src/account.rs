//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    #[sea_orm(column_type = "Text")]
    pub provider: String,
    #[sea_orm(column_name = "providerAccountId", column_type = "Text")]
    pub provider_account_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub access_token: String,
    pub expires_at: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_token: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub token_type: String,
    #[sea_orm(column_type = "Text")]
    pub scope: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub session_state: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub oauth_token_secret: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub oauth_token: Option<String>,
    #[sea_orm(column_name = "userId")]
    pub user_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}