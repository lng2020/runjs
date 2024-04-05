use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::{futures::FutureExt, op2, ModuleLoadResponse, ModuleType};
use std::{ffi::OsStr, path::Path, rc::Rc};
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

struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::error::AnyError> {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        __requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let fut = async move {
          let path = module_specifier.to_file_path().unwrap();
          let (module_type, should_transpile ) = match path.extension().and_then(OsStr::to_str){
            Some(ext) => {
              match ext {
                "ts" | "tsx" => (ModuleType::JavaScript, true),
                "js" | "mjs" => (ModuleType::JavaScript, false),
                _ => panic!("Invalid extension")
              }
            }
            None => panic!("Invalid extension")
          };
    
          let code = std::fs::read_to_string(&path)?;
          let code = if should_transpile {
            let parsed = deno_ast::parse_module(ParseParams {
                specifier: module_specifier.clone(),
                text_info: SourceTextInfo::from_string(code),
                media_type: MediaType::TypeScript,
                capture_tokens: false,
                maybe_syntax: None,
                scope_analysis: false,
            })?;
            parsed.transpile(&Default::default())?.text
          } else {
            code
          };

          let module = deno_core::ModuleSource::new(
            module_type,
            deno_core::ModuleSourceCode::Bytes(code.into_bytes().into_boxed_slice().into()),
            &module_specifier,
            None,
          );
          Ok(module)
        }
        .boxed_local();
        ModuleLoadResponse::Async(fut)
    }
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, Path::new("/home/lng2020/runjs"))?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
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
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2{
        eprintln!("Usage: runjs <file_path>");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(err) = runtime.block_on(run_js(file_path)) {
        eprintln!("error: {err}");
    }
}
