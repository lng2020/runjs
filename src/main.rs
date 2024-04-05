use deno_core::op2;
use std::{path::Path, rc::Rc};
use deno_core::error::AnyError;

deno_core::extension!(
    my_extension,
    ops = [
        op_read_file,
        op_write_file,
        op_remove_file,
        op_fetch
    ],
    esm_entry_point = "ext:my_extension/extension.js",
    esm = [dir "", "extension.js"]
);

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
#[string]
async fn op_write_file(#[string] path: String,#[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, Path::new("/home/lng2020/runjs"))?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![my_extension::init_ops_and_esm()], 
        ..Default::default()
    });
    js_runtime.execute_script("[runjs:runtime.js]", include_str!("/home/lng2020/runjs/runtime.js")).unwrap();

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(err) = runtime.block_on(run_js("example.js")) {
        eprintln!("{}", err);
    }
}
