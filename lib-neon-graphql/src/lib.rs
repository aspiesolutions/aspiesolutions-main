#[macro_use]
extern crate juniper;
pub mod graphql;
use std::{collections::HashMap, default};

use graphql::{Mutation, Query, Schema};
use juniper::{EmptySubscription, InputValue};
use neon::prelude::*;

fn get_sdl(mut cx: FunctionContext) -> JsResult<JsString> {
    let sdl = Schema::new(Query,Mutation,EmptySubscription::<()>::default()).as_schema_language();
    Ok(cx.string(sdl))
}
fn execute_juniper(mut cx: FunctionContext) -> JsResult<JsString> {
    let source_jsstring: Handle<JsString> = cx.argument::<JsString>(0)?;
    let source = source_jsstring.value(&mut cx);

    // let source_string = format!("{}",source_jsstring::);
    let variables: HashMap<String, InputValue> = HashMap::default();
    let context: graphql::Context = graphql::Context {};
    let schema = Schema::new(Query, Mutation, EmptySubscription::default());
    let execution_result = juniper::execute_sync(source.as_str(), None, &schema, &variables, &());
    match execution_result {
        Ok(r) => {
            // let value = r.0;
            // let executionErrors = r.1;
            // // executionErrors.to
            // let tuple = cx.empty_array();
            match serde_json::to_string(&r) {
                Ok(s) => Ok(cx.string(s)),
                Err(e) => cx.throw_error(e.to_string()),
            }
        }
        Err(e) => cx.throw_error(&format!("GraphQLError {}", e)),
    }
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("execute_juniper", execute_juniper)?;
    Ok(())
}
