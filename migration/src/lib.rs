pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user;
mod m20220710_115953_create_session;
mod m20220710_122030_create_account;
mod m20220724_163650_create_transaction;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user::Migration),
            Box::new(m20220710_115953_create_session::Migration),
            Box::new(m20220710_122030_create_account::Migration),
            Box::new(m20220724_163650_create_transaction::Migration),
        ]
    }
}
