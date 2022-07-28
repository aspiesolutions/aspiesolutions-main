use std::str::FromStr;

use access_code::AccessCode;
use juniper::graphql_object;
use juniper::FieldResult;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;

pub use juniper::EmptySubscription;
use juniper::ID;

pub mod access_code;
pub mod address;
pub mod node;
pub mod pageinfo;
pub mod session;
pub mod user;

use node::NodeValue;
// this module contains our graphql api
pub struct Context {
    pub conn: sea_orm::DatabaseConnection,
    pub auth: Option<AuthContext>,
}
pub struct AuthContext {
    token: Option<String>,
    claims: aspiesolutions_core::auth0::TokenClaims,
    subject: user,
}

impl juniper::Context for Context {}
pub struct Query;
#[graphql_object(context=Context)]
impl Query {
    pub async fn node<'context>(
        id: ID,
        context: &'context Context,
    ) -> juniper::FieldResult<Option<NodeValue>> {
        use sea_orm::prelude::Uuid;
        // implmenting the node interface is tricky
        // we have to query all types that node implments and return the first that matches
        // the database uses uuids
        let uuid = Uuid::from_str(&*id)?;

        let results = tokio::try_join!(
            entity::user::Entity::find_by_id(uuid).one(&context.conn),
            entity::session::Entity::find_by_id(uuid).one(&context.conn)
        )?;
        // we only care about the some cases
        match results {
            (Some(user), _) => return Ok(Some(NodeValue::User(user.into()))),
            (_, Some(session)) => return Ok(Some(NodeValue::Session(session.into()))),
            _ => Ok(None),
        }
    }
    pub fn access_code(_id: ID) -> Option<AccessCode> {
        None
    }
    pub async fn user<'context>(
        _id: ID,
        context: &'context Context,
    ) -> juniper::FieldResult<Option<user::User>> {
        // we need to know who is performing this action
        use sea_orm::prelude::Uuid;
        if context.auth.is_none() {
            return Err(aspiesolutions_core::Error::Unauthorized(
                "Permission denied. no auth context present".to_string(),
            ));
        }
        let auth_context = context.auth.unwrap();
        let subject_model = match entity::user::Entity::find()
            .filter(entity::user::Column::IdpId.eq(auth_context.claims.sub))
            .one(&context.conn)
            .await?
        {
            Some(subject) => subject,
            None => {
                return Err(aspiesolutions_core::Error::UserNotFoundError(format!(
                    "with idp_id of {}",
                    auth_context.claims.sub
                )));
            }
        };

        // try to parse the id into a uuid
        let uuid = Uuid::from_str(&*_id)?;
        // if that works, then try to find the entity
        let object = match entity::user::Entity::find_by_id(uuid)
            .one(&context.conn)
            .await?
        {
            Some(o) => o,
            None => {
                return Err(aspiesolutions_core::Error::UserNotFoundError(format!(
                    "with id {uuid}"
                )))
            }
        };
        let meta_permissions = std::collections::HashMap::new();
        let scopes = context.auth.unwrap().claims.scope.unwrap_or(String::new());
        let access_decision: aspiesolutions_core::permissions::Access =
            aspiesolutions_core::permissions::enforce_access_async::<
                entity::user::Model,
                entity::user::Model,
            >(subject, &object, action, scopes.as_str(), meta_permissions)
            .await?;
        match access_decision {
            aspiesolutions_core::permissions::Access::Allow => Ok(Some(user::User {
                id: ID::new(subject_model.id().to_string()),
                idp_id: subject_model.idp_id,
            })),
            aspiesolutions_core::permissions::Access::Deny => {
                Err(aspiesolutions_core::Error::Unauthorized(String::from(
                    "You do not have permission to perform this action. No other reason given",
                )))
            }
        }
        // let user_opt = user::User::map_model_opt(model_opt);
    }
}
pub fn try_authenticate<T: aspiesolutions_core::StructNameSnakeCase>(
    op: &str,
    context: &Context,
) -> Result<(), aspiesolutions_core::Error> {
    if context.auth.is_none() {
        return Err(aspiesolutions_core::Error::Unauthorized(
            "Authorization Context missing in request".to_string(),
        ));
    }
    let auth_context = context.auth.as_ref().unwrap();
    if auth_context.claims.is_none() {
        return Err(aspiesolutions_core::Error::Unauthorized(
            "Missing Authorization claims in request".to_string(),
        ));
    }
    let claims = auth_context.claims.as_ref().unwrap();
    Ok(())
}
pub struct Mutation;
#[graphql_object(context=Context)]
impl Mutation {
    pub fn noop() -> Option<String> {
        None
    }
    // create the user from the auth context
    pub async fn create_user<'context>(context: &'context Context) -> FieldResult<user::User> {
        try_authenticate::<user::User>("create", context)?;
        todo!()
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
use sea_orm::QueryFilter;

#[cfg(test)]
pub mod test {
    use super::AuthContext;
    use aspiesolutions_core::constants::ENV_KEY_DATABASE_URL;
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
        let schema = Schema::new(
            crate::Query,
            crate::Mutation,
            crate::EmptySubscription::<crate::Context>::default(),
        );
        let conn = sea_orm::Database::connect(std::env::var(ENV_KEY_DATABASE_URL).unwrap())
            .await
            .unwrap();
        let context = crate::Context {
            conn,
            auth: Some(AuthContext {
                token: None,
                claims: None,
            }),
        };
        let variables: HashMap<String, InputValue> = HashMap::new();
        let execution_result = juniper::execute(query, None, &schema, &variables, &context)
            .await
            .expect("Query Failed");
        println!("{ex&ecution_result:#?}");
    }
}
