use entity::prelude::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveEnum, ConnectionTrait, DatabaseBackend, Schema, Statement},
};
// this script creates the Financial Planner Bank Account in the database if it does not exist.
// it drops the table

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        // this table requires an enum
        let typ_name_exists_stmt = Statement::from_string(
            DatabaseBackend::Postgres,
            format!(
                r#"select EXISTS(SELECT 1 from "pg_type" where "typname" = '{}') as "exists";"#,
                FPBAType::name()
            ),
        );
        let conn = manager.get_connection();

        let enum_exists: bool = conn
            .query_one(typ_name_exists_stmt)
            .await?
            .unwrap()
            .try_get("", "exists")?;
        if !enum_exists {
            manager
                .create_type(
                    schema
                        .create_enum_from_active_enum::<FPBAType>()
                        .values(vec![FPBAType::Checking, FPBAType::Savings])
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                schema
                    .create_table_from_entity(FinancialPlannerBankAccount)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                schema
                    .create_table_from_entity(FinancialPlannerTransaction)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(
                Table::drop()
                    .table(FinancialPlannerBankAccount)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;
        // I dont know how to say drop type 'type' in the query builder so im using raw sql here and it works just fine
        manager
            .get_connection()
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                format!(r#"DROP TYPE IF EXISTS "{}""#, FPBAType::name()),
            ))
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(FinancialPlannerTransaction)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
