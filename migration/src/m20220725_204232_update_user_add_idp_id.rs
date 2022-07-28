use entity::user;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(user::Column::IdpId)
                            .text()
                            .unique_key()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Table::alter()
            .table(user::Entity)
            .drop_column(user::Column::IdpId)
            .to_owned();
        manager.alter_table(stmt).await
    }
}
