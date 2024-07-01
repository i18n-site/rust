use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;
use i18::i18n_conf_path;
use i18n_js::Conf;

use crate::run;

pub async fn cli() -> Result<()> {
  if let Some((m, _)) = cmdv!(
    i18::purge_arg!(),
    arg!(-d --dir [PATH] "workdir"),
    arg!(-n --npm "publish to npm ( default htm_conf is .i18n/htm/ol.yml )"),
    arg!(-c --htm_conf [CONF] "use which .i18n/htm/ conf ( default is dev.yml )"),
    arg!(-o --ol "use ol htm conf & publish to npm"),
    arg!(-s --save "save & update root version"),
  ) {
    let workdir = m
      .get_one("dir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let purge = i18::purge::is(&m);

    for dir in i18::find_i18n_dir_or_exit(&workdir) {
      println!("❯ {}", dir.display());
      let conf: Conf = yconf::load_or_exit(&i18n_conf_path(&dir));
      if purge {
        i18::purge::purge(&dir, &conf.i18n.fromTo)?;
      } else {
        run(dir, conf, &m).await?;
      }
    }
  }
  OK
}
