use aok::{Result, OK};

genv::def!(CARGO_MANIFEST_DIR);

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();

  // config.type_attribute(".", "#[derive(serde::Serialize)]");
  // config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
  config.compile_protos(&["api.proto"], &[CARGO_MANIFEST_DIR::<String>()])?;

  OK
}
