use std::path::Path;

use aok::{Null, OK};

use crate::{conf::HtmConf, DOT_I18N, HTM};

pub async fn worker(root: &Path, conf: &HtmConf, upload: &impl ckv::Ckv) -> Null {
  let htm = root.join(DOT_I18N).join(HTM);
  for (file, out) in [("serviceWorker.js", "S.js"), ("sharedWorker.js", "W.js")] {
    let fp = htm.join(file);
    if fp.exists() {
      let mut js = minjs::file(&fp)?
        .replace("{conf.v}", &conf.v)
        .replace("{conf.x}", &conf.x);

      if let Some(api) = &conf.api {
        js = js.replace("{conf.api}", api);
      }

      let js = minjs::minjs(&js)?;
      upload.put(out, js).await?;
      // ifs::wstr(outdir.join(out), js)?;
    }
  }
  OK
}
