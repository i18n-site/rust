use std::{env, path::PathBuf};

use aok::{Result, OK};
use regex::Regex;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();

  config.compile_protos(&["api.proto"], &[ROOT])?;

  let out: PathBuf = env::var("OUT_DIR")?.into();

  let root: PathBuf = ROOT.into();
  let sql = ifs::rtxt(root.join("src/init.sql"))?.replace("\n", " ");

  let re = Regex::new(r"\s+").unwrap();
  let sql = re.replace_all(&sql, " ");

  let re = Regex::new(r"\s*([!=:()|;+.])\s*").unwrap();
  let sql = re.replace_all(&sql, "$1");

  ifs::wtxt(
    out.join("INIT_SQL.rs"),
    format!("pub const INIT_SQL:&str = r###\"{sql}\"###;"),
  )?;
  OK
}
