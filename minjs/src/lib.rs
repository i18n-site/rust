use std::sync::Arc;

use aok::Result;
use swc::{self, config, try_with_handler, BoolConfig, BoolOrDataConfig};
use swc_common::{SourceMap, GLOBALS};
use swc_ecma_ast::EsVersion;

pub fn minjs(code: impl Into<String>) -> Result<String> {
  let code = code.into();

  let cm = Arc::<SourceMap>::default();

  let opts = config::Options {
    config: config::Config {
      minify: BoolConfig::new(Some(true)),
      jsc: config::JscConfig {
        target: Some(EsVersion::EsNext),
        minify: Some(config::JsMinifyOptions {
          compress: BoolOrDataConfig::from_bool(true),
          mangle: BoolOrDataConfig::from_bool(true),
          ..Default::default()
        }),
        ..Default::default()
      },
      ..Default::default()
    },
    ..Default::default()
  };

  let c = swc::Compiler::new(cm.clone());
  let output = GLOBALS.set(&Default::default(), || {
    try_with_handler(cm.clone(), Default::default(), |handler| {
      let fm = cm.new_source_file(swc_common::source_map::FileName::Anon, code);
      c.process_js_file(fm, handler, &opts)
    })
  })?;

  Ok(output.code)
}
