use std::str::FromStr;

// the next line derives debug only when not release
// #[cfg_attr(any(test,debug_assertions,feature="enable_derive_debug",derive(Debug)))]
use access_code::AccessCode;
use aspiesolutions_core::constants::scopes::SCOPE_READ_USER;
use juniper::graphql_object;
use juniper::FieldResult;
use sea_orm::EntityTrait;
use user::GetUserResult;

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
#[cfg_attr(
    any(test, debug_assertions, feature = "enable_derive_debug"),
    derive(Debug)
)]
#[derive(Clone)]
pub struct Context {
    pub conn: sea_orm::DatabaseConnection,
    pub auth: Option<AuthContext>,
}
#[derive(Clone, Default)]
#[cfg_attr(
    any(test, debug_assertions, feature = "enable_derive_debug"),
    derive(Debug)
)]
pub struct AuthContext {
    // pub token: Option<String>,
    pub claims: aspiesolutions_core::auth0::TokenClaims,
    // pub user: crate::user::User,
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
            (Some(user), _) => Ok(Some(NodeValue::User(user.into()))),
            (_, Some(session)) => Ok(Some(NodeValue::Session(session.into()))),
            _ => Ok(None),
        }
    }
    pub fn access_code(_id: ID) -> Option<AccessCode> {
        None
    }
    pub async fn user<'context>(
        _id: ID,
        context: &'context Context,
    ) -> juniper::FieldResult<GetUserResult> {
        // we need to know who is performing this action
        use sea_orm::prelude::Uuid;
        if context.auth.is_none() {
            log::error!("Query user: auth context is None. returning null");
            return Ok(GetUserResult {
                user: None,
                errors: vec![String::from("Authorization Required")],
            });
        }
        let scopes = context.auth.as_ref().unwrap().claims.scope.as_ref();
        // check required scopes
        if scopes.is_none() || !scopes.unwrap().contains(SCOPE_READ_USER) {
            return Ok(GetUserResult {
                user: None,
                errors: vec![String::from(
                    "You dont have permission to perform this action",
                )],
            });
        }
        // try to parse the id into a uuid
        let uuid = Uuid::from_str(&*_id)?;
        // if that works, then try to find the entity
        let user: Option<crate::user::User> = entity::user::Entity::find_by_id(uuid)
            .one(&context.conn)
            .await?
            .map(|m| m.into());

        // let meta_permissions = std::collections::HashMap::new();

        Ok(GetUserResult {
            user,
            errors: vec![],
        })
        // let user_opt = user::User::map_model_opt(model_opt);
    }
}
pub struct Mutation;
#[graphql_object(context=Context)]
impl Mutation {
    pub fn noop() -> Option<String> {
        None
    }
    // create the user from the auth context
    pub async fn create_user<'context>(_context: &'context Context) -> FieldResult<user::User> {
        // try_authenticate::<user::User>("create", context)?;
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
    pub async fn test_get_user() {
        let query = r#"query testQuery
        {
            user(id:"627ecff7-a969-4e9a-b433-ad8e61154cee") {
                user {
                    id
                }
                errors
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
                // token: None,
                claims: Default::default(),
            }),
        };
        let variables: HashMap<String, InputValue> = HashMap::new();
        let execution_result = juniper::execute(query, None, &schema, &variables, &context).await;
        // we should not error unless its unrecoverable
        assert_eq!(execution_result.is_ok(), true);
        let (_value, errors) = execution_result.unwrap();
        assert_eq!(errors.len(), 0);
        // .expect("Query Failed");
    }
}
