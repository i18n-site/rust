use std::path::Path;

use aok::{Void, OK};

use crate::{conf::HtmConf, DOT_I18N, HTM};
pub const HOST_JSD: &str = "fastly.jsdelivr.net/npm>cdn.jsdmirror.cn/npm>unpkg.com>jsd.onmicrosoft.cn/npm>cdn.jsdelivr.net/npm>jsd.cdn.noisework.cn/npm>quantil.jsdelivr.net/npm";

pub const HOST_V: &str = "v.ok0.pw>v.3ti.site>v.i18n.site";

pub async fn worker(root: &Path, conf: &HtmConf, upload: &impl ckv::Ckv) -> Void {
  let dir = root.join(DOT_I18N);
  let htm = dir.join(HTM);

  let cdn_jsd = if conf.cdn.jsd.is_empty() {
    HOST_JSD.into()
  } else {
    conf.cdn.jsd.join(">")
  };

  let cdn_v = if conf.cdn.v.is_empty() {
    HOST_V.into()
  } else {
    conf.cdn.v.join(">")
  };

  for (file, out) in [("serviceWorker.js", "S.js"), ("sharedWorker.js", "W.js")] {
    let fp = htm.join(file);
    if fp.exists() {
      let mut js = minjs::file(&fp)?
        .replace("{pkg.i}", &conf.pkg.i)
        .replace("{pkg.md}", &conf.pkg.md)
        .replace("{cdn.v}", &cdn_v)
        .replace("{cdn.jsd}", &cdn_jsd);

      if let Some(api) = &conf.api {
        js = js.replace("{conf.api}", api);
      }

      let js = minjs::minjs(&js)?;
      upload.put(out, js).await?;
    }
  }
  OK
}
