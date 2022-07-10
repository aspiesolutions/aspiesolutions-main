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
        todo!()
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
