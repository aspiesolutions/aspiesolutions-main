use sea_orm::prelude::*;
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "financial_planner_bill_due_date")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    #[sea_orm(db_type = "uuid")]
    bill_id: Uuid,
    #[sea_orm(db_type = "uuid")]
    due_on: ChronoDateTimeUtc,
    planned_due_amount: Decimal,
}

#[derive(EnumIter, Debug, PartialEq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bills::Entity",
        from = "Column::BillId",
        to = "super::bills::Column::Id"
    )]
    FinancialPlannerBill,
}
impl Related<super::bills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinancialPlannerBill.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
