use entity::prelude::*;
use sea_orm::prelude::Uuid;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DatabaseBackend, Schema},
    sea_query::extension::postgres::Type,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DatabaseBackend::Postgres);
        manager
            .create_table(
                schema
                    .create_table_from_entity(FinancialPlannerBankAccount)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let schema: Schema = Schema::new(DatabaseBackend::Postgres);
        manager
            .drop_table(
                Table::drop()
                    .table(FinancialPlannerBankAccount)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await

        // manager
        //     .drop_table(Table::drop().table(Post::Table).to_owned())
        //     .await
    }
}
