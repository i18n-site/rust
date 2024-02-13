genv::def!(CARGO_MANIFEST_DIR);
use aok::{Result, OK};

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();

  // config.type_attribute(".", "#[derive(serde::Serialize)]");
  // config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
  config.compile_protos(&["api.proto"], &[CARGO_MANIFEST_DIR::<String>()])?;

  OK
}
