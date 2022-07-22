use entity::account;
use entity::user;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220710_122030_create_account"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(account::Entity)
                    .col(
                        ColumnDef::new(account::Column::Id)
                            .uuid()
                            .primary_key()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(account::Column::Type).text().not_null())
                    .col(ColumnDef::new(account::Column::Provider).text().not_null())
                    .col(ColumnDef::new(account::Column::ProviderAccountId).text().not_null())
                    .col(ColumnDef::new(account::Column::RefreshToken).text())
                    .col(ColumnDef::new(account::Column::AccessToken).text().not_null())

                    .col(ColumnDef::new(account::Column::AccessTokenExpiresAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(account::Column::IdToken).text())
                    .col(ColumnDef::new(account::Column::TokenType).text().not_null())
                    .col(ColumnDef::new(account::Column::Scope).text().not_null())
                    .col(ColumnDef::new(account::Column::SessionState).text())
                    .col(ColumnDef::new(account::Column::UserId).uuid().not_null())
                    .to_owned(),
            )
            .await?;
            manager.create_foreign_key(ForeignKey::create().from_tbl(account::Entity).from_col(account::Column::UserId).to_tbl(user::Entity).to_col(user::Column::Id).to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(account::Entity).to_owned()).await
    }
}
