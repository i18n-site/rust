use std::{env, path::PathBuf};

use pilota_build::{Builder, IdlService};

fn main() {
  let current_dir = env::current_dir().expect("Failed to get current directory");
  let proto_dir = current_dir.join("proto");
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

  Builder::pb()
    .include_dirs(vec![proto_dir.clone()])
    .compile_with_config(
      vec![IdlService::from_path(proto_dir.join("api.proto"))],
      pilota_build::Output::File(PathBuf::from(out_dir).join("api.rs")),
    );
}
