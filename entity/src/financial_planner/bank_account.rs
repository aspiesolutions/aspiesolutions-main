use sea_orm::prelude::*;
use sea_orm::sea_query;
/// Models a bank account to use as a reference to a real bank account
/// for a financial planner.

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bank_account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    created: ChronoDateTimeUtc,
    starting_balance_when_created: Decimal,
    account_type: AccountType,
    user_id: Uuid,
}
#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user::Entity",
        from = "Column::UserId",
        to = "crate::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::bills::Entity")]
    FinancialPlannerBills,
    #[sea_orm(has_many = "super::transaction::Entity")]
    FinancialPlannerTransaction,
}
#[derive(EnumIter, DeriveActiveEnum, Iden, Debug, Clone, PartialEq)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "financial_planner_bank_account_enum"
)]
pub enum AccountType {
    #[sea_orm(string_value = "Checking")]
    Checking,
    #[sea_orm(string_value = "Savings")]
    Savings,
}
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerTransaction.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
