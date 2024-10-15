use std::{path::Path, sync::Arc};

use aok::Result;
use swc::{self, config, try_with_handler, BoolConfig, BoolOrDataConfig};
use swc_common::{SourceMap, GLOBALS};
use swc_ecma_ast::EsVersion;

pub fn file(fp: &Path) -> Result<String> {
  match minjs(ifs::rstr(fp)?) {
    Ok(js) => Ok(js),
    Err(e) => {
      tracing::error!("❌ {:?}", fp);
      Err(e)
    }
  }
}

fn _minjs(code: impl Into<String>, mangle: bool) -> Result<String> {
  let code = code.into();

  let cm = Arc::<SourceMap>::default();

  let opts = config::Options {
    config: config::Config {
      minify: BoolConfig::new(Some(true)),
      jsc: config::JscConfig {
        target: Some(EsVersion::EsNext),
        minify: Some(config::JsMinifyOptions {
          mangle: BoolOrDataConfig::from_obj(swc_ecma_minifier::option::MangleOptions {
            top_level: Some(mangle),
            ..Default::default()
          }),
          compress: BoolOrDataConfig::from_obj(
            swc_ecma_minifier::option::terser::TerserCompressorOptions {
              toplevel: Some(
                swc_ecma_minifier::option::terser::TerserTopLevelOptions::Bool(mangle),
              ),
              // reduce_vars: Some(false),
              // reduce_funcs: Some(false),
              drop_console: false,
              unused: Some(mangle),
              side_effects: Some(false),
              ..Default::default()
            },
          ),
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
      let fm = cm.new_source_file(swc_common::source_map::FileName::Anon.into(), code);
      c.process_js_file(fm, handler, &opts)
    })
  })?;

  Ok(output.code)
}

pub fn minjs_no_mangle(code: impl Into<String>) -> Result<String> {
  _minjs(code, false)
}

pub fn minjs(code: impl Into<String>) -> Result<String> {
  _minjs(code, true)
}
