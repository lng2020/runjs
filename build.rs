use std::io::Write;
use std::{env, vec};
use std::path::PathBuf;

use deno_core::snapshot::{create_snapshot, CreateSnapshotOptions};

fn main() {
    deno_core::extension!(
        my_extension,
        js = ["src/extension.js",]
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("snapshot.bin");
    let output = create_snapshot(
        CreateSnapshotOptions {
          cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
          startup_snapshot: None,
          extensions: vec![my_extension::init_ops_and_esm()],
          extension_transpiler: None,
          with_runtime_cb: None,
          skip_op_registration: false,
        },
        None,
      )
      .unwrap();
    let mut file = std::fs::File::create(snapshot_path).unwrap();
    file.write_all(&output.output).unwrap();

    for path in output.files_loaded_during_snapshot {
      println!("cargo:rerun-if-changed={}", path.display());
    }
}