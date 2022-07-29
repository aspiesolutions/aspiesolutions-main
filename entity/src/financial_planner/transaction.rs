use sea_orm::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    bank_account_id: Uuid,
}
#[derive(Clone, EnumIter, Debug, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    FinancialPlannerBankAccount,
}

impl sea_orm::Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBankAccount.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
