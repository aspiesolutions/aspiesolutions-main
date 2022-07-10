use neon::prelude::*;

// we depend on a tokio runtime to execute database queries
lazy_static::lazy_static!{
    pub static ref TOKIO_RUNTIME: std::sync::Arc<tokio::runtime::Runtime> = std::sync::Arc::new(tokio::runtime::Builder::new_current_thread().build().unwrap());
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // start the runtime when the module is loaded
    TOKIO_RUNTIME.block_on(async {println!("future works!")});
    Ok(())
}
