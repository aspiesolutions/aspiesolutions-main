use sea_orm::prelude::*;
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bills")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    #[sea_orm(db_type = "uuid")]
    bank_account_id: Uuid,
    #[sea_orm(db_type = "uuid")]
    payment_id: Uuid,
    #[sea_orm(db_type = "uuid")]
    user_id: Uuid,
    #[sea_orm(db_type = "uuid")]
    due_on: ChronoDateTimeUtc,
    planned_due_amount: Decimal,
}

#[derive(EnumIter, Debug, PartialEq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    FinancialPlannerBankAccount,
    #[sea_orm(has_many = "super::bill_payment::Entity")]
    FinancialPlannerBillPayment,
    #[sea_orm(
        belongs_to = "crate::user::Entity",
        from = "Column::UserId",
        to = "crate::user::Column::Id"
    )]
    User,
}
impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBankAccount.def()
    }
}
impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
