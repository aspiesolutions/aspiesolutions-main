use sea_orm::prelude::*;
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bills")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    bank_account_id: Uuid,
}

#[derive(EnumIter, Debug, PartialEq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    FinancialPlannerBankAccount,
}
impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBankAccount.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
