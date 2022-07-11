use neon::prelude::*;

// we depend on a tokio runtime to execute database queries
lazy_static::lazy_static!{
    pub static ref TOKIO_RUNTIME: std::sync::Arc<tokio::runtime::Runtime> = std::sync::Arc::new(tokio::runtime::Builder::new_current_thread().build().unwrap());
}

fn create_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    let o = cx.empty_object();
    println!("Stubbed create user");
    Ok(o)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // start the runtime when the module is loaded
    cx.export_function("createUser", create_user)?;
    Ok(())
}
