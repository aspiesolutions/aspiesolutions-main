use sea_orm_migration::{prelude::*, sea_orm::Schema};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let schema = Schema::new(manager.get_database_backend());
        manager
            .create_table(
                schema
                    .create_table_from_entity(entity::prelude::FinancialPlannerBill)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                schema
                    .create_table_from_entity(entity::prelude::FinancialPlannerBillAmountDue)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                schema
                    .create_table_from_entity(entity::prelude::FinancialPlannerBillPayment)
                    .to_owned(),
            )
            .await?;
        Ok(())
        // .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(entity::prelude::FinancialPlannerBillPayment)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(entity::prelude::FinancialPlannerBillAmountDue)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(entity::prelude::FinancialPlannerBill)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
