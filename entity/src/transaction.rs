use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// identifies this transaction
    id: Uuid,
    /// where did the money come from
    originator_id: Uuid,
    /// who or what was the money sent to
    reciever_id: Uuid,
    /// who or what performed the transaction
    sender_id: Option<Uuid>,
    /// where was the money sent from
    originating_location: Option<String>,
    /// where was the money sent to
    recieving_location: Option<String>,
    /// where was the location of the person or thing who performed this transaction
    sending_location: Option<String>,
    /// what was the money spent on
    items: Option<String>,
    /// the total cost/gain of this transaction
    amount: i64,
    // what currency was the transaction performed in
    originating_currency: String,
    // the currency of the reciever. used to allow conversion to/from external currencies
    recieving_currency: String,
    // what account the money was sent from
    originator_account: String,
    // what account the money was sent to
    reciever_account: String,
    // used to organize transactions
    categories: Option<String>,
    // when was the transaction created
    created_date: DateTimeWithTimeZone,
    // when was the transaction finalized
    finalized_date: Option<DateTimeWithTimeZone>,
    // whether the transaction is pending, returned, or complete
    status: String,
    // whether the transaction was handled with card,cash or ACH
    method: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
