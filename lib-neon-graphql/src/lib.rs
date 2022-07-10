#![recursion_limit = "1024"]
#[macro_use]
extern crate juniper;
pub mod graphql;
use std::collections::HashMap;

use graphql::{Mutation, Query, Schema};
use juniper::{EmptySubscription, InputValue, ExecutionError, DefaultScalarValue};
use neon::prelude::*;

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

// fn serde_json_map_str_value_to_neon_object<'a>(
//     mut cx: &mut FunctionContext<'a>,
//     map: &serde_json::Map<String, serde_json::Value>,
// ) -> JsResult<'a, JsObject> {
//     let o = cx.empty_object();
//     for (key, value) in map.iter() {
//         if let serde_json::Value::Object(map) = value {
//             let n_o = serde_json_map_str_value_to_neon_object(&mut cx, map)?;
//             o.set::<FunctionContext, &str, JsObject>(&mut cx, key, n_o)?;
//         } else if let serde_json::Value::Array(arr) = value {
//             let a = serde_json_array_of_values_to_neon_array(&mut cx, arr)?;

//             o.set::<FunctionContext, &str, JsArray>(&mut cx, key, a)?;
//         } else if let serde_json::Value::String(serde_string) = value {
//             let s = cx.string(serde_string);
//             o.set::<FunctionContext, &str, JsString>(&mut cx, key, s)?;
//         } else if let serde_json::Value::Number(serde_num) = value {
//             let num_f64 = serde_num.as_f64();
//             if num_f64.is_some() {
//                 let n = cx.number(num_f64.unwrap());
//                 o.set::<FunctionContext, &str, JsNumber>(&mut cx, key, n)?;
//             } else {
//                 return cx.throw_type_error(&format!("Could not convert serde::Value::Number to an f64 while converting a serde::Object(Map<String,Value>) to a neon object!"));
//             }
//         } else if value.is_null() {
//             let null = cx.null();
//             o.set::<FunctionContext, &str, JsNull>(&mut cx, key, null)?;
//         } else if let serde_json::Value::Bool(bool) = value {
//             let js_bool = cx.boolean(*bool);
//             o.set::<FunctionContext, &str, JsBoolean>(&mut cx, key, js_bool)?;
//         }
//     }
//     Ok(o)
// }

fn serde_json_map_to_object<'a>(
    mut cx: &mut FunctionContext<'a>,
    map: &serde_json::Map<String, serde_json::Value>,
) -> JsResult<'a, JsObject> {
    let o = cx.empty_object();
    for (key, value) in map.iter() {
        println!("key: {}, value: {}", key, value);
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
                println!("not parsing arrays currently!");
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
                println!("Object parsing not finished yet!");
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
fn juniper_execution_error_to_neon_primitive<'a>(mut cx: &mut FunctionContext<'a>, error: ExecutionError<DefaultScalarValue>)-> JsResult<'a,JsUndefined> {

    Ok(cx.undefined())
}

fn execute_juniper(mut cx: FunctionContext) -> JsResult<JsObject> {
    let arguments = cx.argument::<JsObject>(0)?;
    let jsstring_source: Handle<JsString> =
        arguments.get::<JsString, FunctionContext, &'static str>(&mut cx, "source")?;
    let jsstring_operation_name =
        arguments.get_opt::<JsString, FunctionContext, &'static str>(&mut cx, "operationName")?;

    let source = jsstring_source.value(&mut cx);
    let mut operation_name = String::new();
    if jsstring_operation_name.is_some() {
        operation_name = jsstring_operation_name.unwrap().value(&mut cx).to_owned();
    }
    let slice = operation_name.as_str();
    // let source_string = format!("{}",source_jsstring::);
    let variables: HashMap<String, InputValue> = HashMap::default();
    let context: graphql::Context = graphql::Context {};
    let schema = Schema::new(Query, Mutation, EmptySubscription::default());
    let execution_result = juniper::execute_sync(
        source.as_str(),
        match jsstring_operation_name {
            Some(_) => Some(slice),
            None => None,
        },
        &schema,
        &variables,
        &(),
    );
    match execution_result {
        Ok(r) => {
            let value = r.0;
            let executionErrors = r.1;
            // // executionErrors.to
            let o = cx.empty_object();
            let value_map: serde_json::Map<String, serde_json::Value> =
                match serde_json::from_str(value.to_string().as_str()) {
                    Ok(map) => map,
                    Err(e) => return cx.throw_error(e.to_string()),
                };
            let value = serde_json_map_to_object(&mut cx, &value_map)?;
            o.set(&mut cx,"data",value)?;

            match serde_json::to_string(&executionErrors) {
                Ok(s) => {
                    let jssstring_errors = cx.string(s);
                    o.set(&mut cx, "errors", jssstring_errors)?
                }
                Err(e) => return cx.throw_error(e.to_string()),
            };
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
