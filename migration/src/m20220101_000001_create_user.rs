use entity::user;
use entity::user::Model;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user"
    }
}
// initialise the
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create the user table
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(user::Column::Name).text())
                    .col(ColumnDef::new(user::Column::Email).text().not_null().unique_key())
                    .col(ColumnDef::new(user::Column::EmailVerified).timestamp_with_time_zone())
                    .col(ColumnDef::new(user::Column::Image).text()).to_owned(),

            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(user::Entity).to_owned())
            .await
    }
}
