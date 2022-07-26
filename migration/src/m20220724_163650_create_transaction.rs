use entity::transaction;
use sea_orm_migration::{
    prelude::*,
    sea_orm::prelude::{ChronoDateTimeUtc, DateTimeUtc, DateTimeWithTimeZone},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(transaction::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(transaction::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::OriginatorId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::RecieverId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(transaction::Column::SenderId).uuid().null())
                    .col(
                        ColumnDef::new(transaction::Column::OriginatingLocation)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::RecievingLocation)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::SendingLocation)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(transaction::Column::Items).text().null())
                    .col(
                        ColumnDef::new(transaction::Column::Categories)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::FinalizedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Status)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Method)
                            .text()
                            .not_null(),
                    )
                    // .col(ColumnDef::new(Post::Title).string().not_null())
                    // .col(ColumnDef::new(Post::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(transaction::Entity).to_owned())
            .await
    }
}
