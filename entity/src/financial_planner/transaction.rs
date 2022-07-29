use sea_orm::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_transaction")]
pub struct Model {
    // uuid does not support auto increment
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    #[sea_orm(db_type = "uuid")]
    bank_account_id: Uuid,
    #[sea_orm(db_type = "uuid")]
    user_id: Uuid,
    /// decimal is chosen here because of the sensitive nature of money
    /// and we need to store large values without crashing.
    ///
    /// The decimal type's MAX value is the biggest number that is
    /// suported by the orm
    amount: Decimal,
}
#[derive(Clone, EnumIter, Debug, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    FinancialPlannerBankAccount,
    #[sea_orm(
        belongs_to = "crate::user::Entity",
        from = "Column::UserId",
        to = "crate::user::Column::Id"
    )]
    User,
}

impl sea_orm::Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBankAccount.def()
    }
}
impl sea_orm::Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
