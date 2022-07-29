use sea_orm::prelude::*;

/// a model used to track when a bill was paid and how much was paid
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bill_payment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    #[sea_orm(db_type = "uuid")]
    bill_id: Uuid,
    amount: Decimal,
    paid_on: ChronoDateTimeUtc,
}
#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bills::Entity",
        from = "Column::BillId",
        to = "super::bills::Column::Id"
    )]
    FinancialPlannerBills,
}
impl Related<super::bills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBills.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
