use std::str::FromStr;

use access_code::AccessCode;
use address::Address;
use juniper::FieldResult;
use juniper::GraphQLObject;
use juniper::graphql_object;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use session::SessionConnection;
pub use juniper::EmptySubscription;
use juniper::ID;

pub mod session;
pub mod pageinfo;
pub mod node;
pub mod address;
pub mod access_code;
pub mod user;

use node::{NodeValue};
// this module contains our graphql api
pub struct Context {
    pub conn: sea_orm::DatabaseConnection
}

impl juniper::Context for Context {
}
pub struct Query;
#[graphql_object(context=Context)]
impl Query {
    pub async fn  node<'context>(id: ID,context:&'context Context) -> juniper::FieldResult<Option<NodeValue>> {
        use sea_orm::prelude::Uuid;
        // implmenting the node interface is tricky
        // we have to query all types that node implments and return the first that matches
        // the database uses uuids
        let uuid = Uuid::from_str(&*id)?;

        let results =
        tokio::try_join!(
            entity::user::Entity::find_by_id(uuid).one(&context.conn),
            entity::session::Entity::find_by_id(uuid).one(&context.conn)
        )?;
        // we only care about the some cases
        match results {
            (Some(user),_) => {
                return Ok(Some(NodeValue::User(user.into())))
            },
            (_,Some(session))=> {
                return Ok(Some(NodeValue::Session(session.into())))
            },
            _=> Ok(None)
        }

    }
    pub fn access_code(_id:ID) -> Option<AccessCode> {
        None
    }
    pub async fn user<'context>(_id:ID,context:&'context Context) -> juniper::FieldResult<Option<user::User>> {
        use sea_orm::prelude::Uuid;
        // try to parse the id into a uuid
        let uuid = Uuid::from_str(&*_id)?;
        // if that works, then try to find the entity
        let model_opt = entity::user::Entity::find_by_id(uuid).one(&context.conn).await?;
        let user_opt = user::User::map_model_opt(model_opt);
        Ok(user_opt)
    }
}




pub struct Mutation;
#[graphql_object(context=Context)]
impl Mutation {
    pub fn noop() -> Option<String> {
        None
    }
    // create the user from the auth context
    pub async fn create_user<'context>(context: &'context Context) -> FieldResult<user::User> {
        todo!();
        // let mut entity = entity::user::Model::default();
        // let mut active_model = entity.into_active_model();
        // active_model.idp_id = sea_orm::ActiveValue::Set(Some(input.idp_id));

        // entity = active_model.insert(&context.conn).await?;
        // Ok(entity.into())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
pub use juniper::execute;
pub use juniper::execute_sync;

#[cfg(test)]
pub mod test {

    use std::collections::HashMap;

    use juniper::InputValue;

    use crate::Schema;

    #[tokio::test]
    pub async fn test_user_query() {
        let query = r#"query testQuery
        {
            user(id:"627ecff7-a969-4e9a-b433-ad8e61154cee") {
                id
            }
        }
        "#;
        let schema = Schema::new(crate::Query,crate::Mutation,crate::EmptySubscription::<crate::Context>::default());
        let conn = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        let context = crate::Context { conn };
        let variables: HashMap<String,InputValue> = HashMap::new();
        let execution_result = juniper::execute(query, None, &schema, &variables, &context).await.expect("Query Failed");
        println!("{execution_result:#?}");


    }
}