use std::path::PathBuf;

use aok::{Null, OK};
use i18::build_ignore;

use crate::{Conf, VDir};

pub async fn gen(dir: PathBuf, conf: Conf, vdir: VDir, upload_s3: bool) -> Null {
  let ignore = build_ignore(conf.ignore);
  i18::run(&dir, &conf.i18n, &ignore, i18::token()).await?;

  dbg!((vdir.find("doc/xyz"), upload_s3));
  OK
}
