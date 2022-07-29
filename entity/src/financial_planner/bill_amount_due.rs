use sea_orm::prelude::*;
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bills_due_amounts")]
// a model used to track bill due dates over time
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    bill_id: Uuid,
    created: ChronoDateTimeUtc,
    amount: Decimal,
}
#[derive(EnumIter, Debug, PartialEq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bills::Entity",
        from = "Column::BillId",
        to = "super::bills::Column::Id"
    )]
    FinancialPlannerBills,
}
impl ActiveModelBehavior for ActiveModel {}
