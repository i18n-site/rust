use std::path::Path;

use aok::{Null, OK};

use crate::{conf::HtmConf, DOT_I18N, HTM};

const X: &str = "x/";

pub fn worker(root: &Path, conf: &HtmConf, outdir: &Path) -> Null {
  let htm = root.join(DOT_I18N).join(HTM);
  let m = &conf.importmap;
  let x = m.get(X).unwrap();

  for (file, out) in [("serviceWorker.js", "S.js"), ("sharedWorker.js", "W.js")] {
    let fp = htm.join(file);
    if fp.exists() {
      let mut js = minjs::file(&fp)?
        .replace("{conf.v}", &conf.v)
        .replace(
          "{conf.dot_v}",
          &conf
            .dot_v
            .clone()
            .unwrap_or_else(|| format!("{}/.v", &conf.v)),
        )
        .replace("{importmap.x}", x);

      if let Some(api) = &conf.api {
        js = js.replace("{conf.api}", api);
      }

      let js = minjs::minjs(&js)?;
      ifs::wtxt(outdir.join(out), js)?;
    } else {
      let fp = outdir.join(out);
      if fp.exists() {
        std::fs::remove_file(&fp)?;
      }
    }
  }

  OK
}
