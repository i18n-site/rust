use std::path::PathBuf;

use aok::{Null, OK};
use i18::token;
use i18_conf::build_ignore;
use i18n_js::{Build, Conf};

use crate::package_json_ver;

pub const DOT_YML: &str = ".yml";

pub async fn run(dir: PathBuf, conf: Conf, m: &clap::ArgMatches) -> Null {
  let npm: bool = m.get_one("npm").cloned().unwrap_or(false);
  let save: bool = m.get_one("save").cloned().unwrap_or(false);

  let mut htm_conf: String = m
    .get_one("htm_conf")
    .cloned()
    .unwrap_or_else(|| if npm { "ol" } else { "dev" }.into());

  if htm_conf.ends_with(DOT_YML) {
    htm_conf = htm_conf[..DOT_YML.len()].into();
  }

  let token = token();

  let ignore = build_ignore(&conf.ignore);

  let dir_i18n = dir.join(".i18n");

  {
    let gitignore = dir_i18n.join(".gitignore");
    if !gitignore.exists() {
      std::fs::write(&gitignore, "hook/\n")?;
    }
  }

  let pkg_li = npmi::PkgLi::new(dir_i18n.join("hook"), &conf.addon);

  pkg_li.auto().await?;

  let changed = i18::run(&dir, &conf.i18n, &ignore, &token).await?;

  let build = Build::new(
    &dir,
    conf,
    &ignore,
    &htm_conf,
    &pkg_li.dir,
    &pkg_li.rel_li("afterTran.js"),
    &changed,
  )
  .await?;

  let vfs = build.build().await?;
  if npm {
    let package_json = dir
      .join(".i18n/htm")
      .join(format!("{htm_conf}.package.json"));
    if package_json.exists() {
      let package_json = package_json_ver(&package_json, &vfs.ver)?;
      let out = dir.join("out").join(&htm_conf).join("v").join(&*vfs.ver);
      if vfs.has_new() {
        npm::publish(&npm::token(), &out, package_json).await?;
        vfs.save()?;
        refresh_v::url(&token, &build.htm_conf.v, &vfs.ver).await?;
        return OK;
      }
    } else {
      tracing::error!("{:?} NOT EXIST", package_json);
    }
  } else if save {
    vfs.save()?;
  }

  {
    let conf = &build.htm_conf;
    for out in &conf.out {
      if conf.seo {
        crate::seo(
          &conf.host,
          &out,
          &dir,
          &htm_conf,
          &build.lang_li,
          &ignore,
          &changed,
          &build.foot(),
          &conf.x,
        )
        .await?;
      }
    }
  }
  // if let Some(ref out) = build.conf.out
  //   && let Some(conf) = out.get(&htm_conf)
  // {
  // }
  println!("✅ i18n.site build");
  OK
}
