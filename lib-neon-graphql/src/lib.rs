#![recursion_limit = "1024"]
#[macro_use]
extern crate juniper;
pub mod graphql;
use std::collections::HashMap;

use graphql::{Mutation, Query, Schema};
use juniper::{DefaultScalarValue, EmptySubscription, ExecutionError, InputValue};
use neon::prelude::*;
use serde_json::to_value;

lazy_static::lazy_static! {
    pub static ref SCHEMA:Schema = Schema::new(Query, Mutation, EmptySubscription::<()>::default());
}

fn serde_json_array_of_values_to_neon_array<'a>(
    mut cx: &mut FunctionContext<'a>,
    array: &Vec<serde_json::Value>,
) -> JsResult<'a, JsArray> {
    let a = cx.empty_array();
    for (i, value) in array.iter().enumerate() {
        match value {
            serde_json::Value::Object(map) => {
                let o = serde_json_map_to_object(&mut cx, map)?;
                a.set(cx, i as u32, o)?;
            }
            serde_json::Value::Array(array) => {
                let a = serde_json_array_of_values_to_neon_array(&mut cx, array)?;
                a.set(cx, i as u32, a)?;
            }
            serde_json::Value::String(serde_string) => {
                let s = cx.string(serde_string);
                a.set(cx, i as u32, s)?;
            }
            serde_json::Value::Number(serde_num) => {
                let num_f64 = serde_num.as_f64();
                if num_f64.is_some() {
                    let num = cx.number(num_f64.unwrap());
                    a.set(cx, i as u32, num)?;
                } else {
                    return cx.throw_type_error(&format!("Failed to convert a serde_json::Value::Number to an f64 while parsing an array!"));
                }
            }
            serde_json::Value::Bool(serde_bool) => {
                let bool = cx.boolean(*serde_bool);
                a.set(cx, i as u32, bool)?;
            }
            serde_json::Value::Null => {
                let null = cx.null();
                a.set(cx, i as u32, null)?;
            }
        }
    }
    Ok(a)
}

fn serde_json_map_to_object<'a>(
    mut cx: &mut FunctionContext<'a>,
    map: &serde_json::Map<String, serde_json::Value>,
) -> JsResult<'a, JsObject> {
    let o = cx.empty_object();
    for (key, value) in map.iter() {
        match value {
            serde_json::Value::String(serde_string) => {
                let jsstring = cx.string(serde_string);
                o.set::<FunctionContext, &str, JsString>(&mut cx, key, jsstring)?;
            }
            serde_json::Value::Null => {
                let null = cx.null();
                o.set::<FunctionContext, &str, JsNull>(&mut cx, key, null)?;
            }
            serde_json::Value::Bool(bool) => {
                let bool = cx.boolean(*bool);
                o.set::<FunctionContext, &str, JsBoolean>(&mut cx, key, bool)?;
            }
            serde_json::Value::Array(array) => {
                let a = serde_json_array_of_values_to_neon_array(&mut cx, array)?;
                a.set::<FunctionContext, &str, JsArray>(&mut cx, key, a)?;
            }
            serde_json::Value::Number(num) => {
                let num_f64 = num.as_f64();
                if num_f64.is_none() {
                    return cx.throw_type_error(format!("Rust could not convert a number '{}' to an f64 representation while preparing an object to cross the Rust->JS boundary",num));
                }
                let n = cx.number(num_f64.unwrap());
                o.set::<FunctionContext, &str, JsNumber>(&mut cx, key, n)?;
            }
            serde_json::Value::Object(map) => {
                let n_o = serde_json_map_to_object(&mut cx, map)?;
                o.set::<FunctionContext, &str, JsObject>(&mut cx, key, n_o)?;
            }
        }
    }
    Ok(o)
}

fn get_sdl(mut cx: FunctionContext) -> JsResult<JsString> {
    let sdl = Schema::new(Query, Mutation, EmptySubscription::<()>::default()).as_schema_language();
    Ok(cx.string(sdl))
}

fn execute_juniper(mut cx: FunctionContext) -> JsResult<JsObject> {
    // get the arguments object
    let arguments = cx.argument::<JsObject>(0)?;
    // get the source string
    let source: String =
        arguments.get::<JsString, FunctionContext, &'static str>(&mut cx, "source")?.value(&mut cx);
    // get the handle to the operation name
    let jsstring_operation_name =
        arguments.get_opt::<JsString, FunctionContext, &'static str>(&mut cx, "operationName")?;
    // get the handle to the object containing the variables used for this query
    let jsobject_variables =
        arguments.get_opt::<JsObject, FunctionContext, &'static str>(&mut cx, "variables")?;
    // do some tricky work with the operation name.
    // rust doesnt like returning string slices from match statements in this case, so we have to create a default string
    // then detect if the operation name is present
    // then set its value to the string
    // then create a slice
    // then we have to detect if the operation name is present again and give juniper the string
    // otherwise we have to leave it blank

    let mut operation_name = String::new();
    if jsstring_operation_name.is_some() {
        operation_name = jsstring_operation_name.unwrap().value(&mut cx).to_owned();
    }
    let slice = operation_name.as_str();
    // let source_string = format!("{}",source_jsstring::);
    let variables: HashMap<String, InputValue> = HashMap::default();
    let context: Context = graphql::Context {};
    // let schema *SCHEMA;
    let execution_result = juniper::execute_sync(
        source.as_str(),
        match jsstring_operation_name {
            Some(_) => Some(slice),
            None => None,
        },
        &SCHEMA,
        &variables,
        &(),
    );
    match execution_result {
        Ok(r) => {
            let value = r.0;
            let execution_errors = r.1;
            // // executionErrors.to
            let o = cx.empty_object();
            let value_map: serde_json::Map<String, serde_json::Value> =
                match serde_json::from_str(value.to_string().as_str()) {
                    Ok(map) => map,
                    Err(e) => return cx.throw_error(e.to_string()),
                };
            let value = serde_json_map_to_object(&mut cx, &value_map)?;
            o.set(&mut cx, "data", value)?;
            let error_values = match serde_json::to_value(execution_errors) {
                Ok(v) => v,
                Err(e) => return cx.throw_error(e.to_string()),
            };
            if let serde_json::Value::Array(array) = error_values {
                let a = serde_json_array_of_values_to_neon_array(&mut cx, &array)?;
                o.set(&mut cx, "errors", a)?;
            } else {
                println!(
                    "ExectionErrors was not an array while handling the juniper execution result"
                )
            }
            Ok(o)
        }
        Err(e) => cx.throw_error(&format!("GraphQLError {}", e)),
    }
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("execute_juniper", execute_juniper)?;
    cx.export_function("get_sdl", get_sdl)?;
    Ok(())
}
