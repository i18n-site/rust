use std::path::{Path, PathBuf};

use aok::{Null, OK};
use gxhash::HashSet;
use i18::{env::I18N_SITE_YML_PATH, token};
use i18_conf::build_ignore;
use i18n_js::{Build, Conf};
use sver::Ver;
use ver_incr::ver_incr;

use crate::package_json_ver;

pub const DOT_YML: &str = ".yml";

pub async fn run(dir: PathBuf, conf: Conf, m: &clap::ArgMatches) -> Null {
  let npm: bool = m.get_one("npm").cloned().unwrap_or(false);
  let save: bool = m.get_one("save").cloned().unwrap_or(false);

  let mut htm_conf: String = m
    .get_one("htm_conf")
    .cloned()
    .unwrap_or_else(|| if npm { "main" } else { "dev" }.into());

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

  let dir_hook = dir_i18n.join("hook");
  let pkg_li = npmi::PkgLi::new(&dir_hook, &conf.addon);

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

  let htm_index_js: Vec<String> = pkg_li
    .rel_li("htmIndex.js")
    .into_iter()
    .flat_map(|p| ifs::rstr(dir_hook.join(p)))
    .collect();

  let htm_index_js = htm_index_js.join("\n");

  let mut vfs = build.build(None, &htm_index_js).await?;

  let mut refresh_v = None;
  if npm {
    let package_json = dir
      .join(".i18n/htm")
      .join(format!("{htm_conf}.package.json"));

    if package_json.exists() {
      let pkg_name = &build.htm_conf.pkg.md;
      loop {
        let pkg = package_json_ver(pkg_name, &package_json, &vfs.ver)?;
        let out = dir.join("out").join(&htm_conf).join("v").join(&*vfs.ver);
        if vfs.has_new() {
          match npm::publish(&npm::token(), &out, pkg.fp, pkg_name).await? {
            npm::State::VerLow => {
              let ver = ver_incr(&npmv::latest(pkg_name).await?);
              let ver = if Ver::from(&ver) <= Ver::from(&vfs.ver) {
                vfs.ver_next()
              } else {
                ver
              };
              vfs = build.build(Some(ver), &htm_index_js).await?;
              continue;
            }
            npm::State::Ok => {}
          }
          vfs.save()?;
          refresh_v = Some(refresh_v::RefreshV::run(&token, &pkg.name, vfs.ver));
        }
        break;
      }
    } else {
      tracing::error!("{:?} NOT EXIST", package_json);
    }
  } else if save {
    vfs.save()?;
  }

  for kind in &build.htm_conf.out {
    let kind = kind.as_str();
    macro_rules! htm {
      ($ckv:expr) => {
        if let Err(err) = htm(&kind, $ckv, &build, &changed, &dir, &ignore).await {
          eprintln!("❌ {kind} {err:?}");
        }
      };
    }
    match kind {
      "fs" => {
        htm!(ckv::Fs::new(dir.join("out").join(&htm_conf).join("htm")))
      }
      "s3" => {
        htm!(ckv::S3::load(&I18N_SITE_YML_PATH, &build.htm_conf.host)?)
      }
      _ => {
        eprintln!("unknown out {kind}");
        continue;
      }
    };
  }

  if let Some(refresh_v) = refresh_v {
    refresh_v.wait();
  }
  println!("✅ i18n.site build");
  OK
}

pub async fn htm(
  kind: &str,
  upload: impl ckv::Ckv,
  build: &Build,
  changed: &HashSet<String>,
  dir: &Path,
  ignore: &globset::GlobSet,
) -> Null {
  let lang_li = &build.lang_li;
  let foot = build.foot();
  tokio::try_join!(
    crate::seo(
      kind,
      &upload,
      &build.htm_conf,
      dir,
      lang_li,
      ignore,
      changed,
      &foot,
    ),
    build.htm(kind, &upload, lang_li)
  )?;
  OK
}
