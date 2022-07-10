use sea_orm_migration::prelude::*;
use entity::session::Model;
use entity::session;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220710_115953_create_session"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create the table
        manager.create_table(Table::create().table(session::Entity).if_not_exists()
        .col(ColumnDef::new(session::Column::Id).primary_key().uuid().unique_key().not_null())
        .col(ColumnDef::new(session::Column::Expires).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(session::Column::SessionToken).text().not_null())
        .col(ColumnDef::new(session::Column::UserId).uuid()).to_owned()).await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(session::Entity).to_owned()).await
    }
}