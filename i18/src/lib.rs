#![feature(coroutines, coroutine_trait)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
use std::path::{Path, PathBuf};

use aok::{Null, OK};
use globset::GlobSet;
use i18_conf::I18nConf;
use lang::LANG_CODE;
use prepare_li::prepare_li;
use print_err::print_err;

mod conf_from_to;
mod err;
mod fetch_tran;
mod prepare_li;
mod print_err;
mod print_tran_result;
mod save;
mod scan;
mod tran;
mod tzst;
mod wait_tran;
pub use conf_from_to::{conf_from_to, RelFt};
pub use print_tran_result::print_tran_result;
pub use save::Save;
pub mod i18n;
pub mod purge;
pub use err::Err;
pub use fetch_tran::fetch_tran;
pub use scan::Scan;
pub use tran::tran;
pub use wait_tran::wait_tran;

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}
use api::TranedLi;

pub const ASSET_BASE: f64 = 100.0;
pub const COST_BASE: f64 = 1000.0 * ASSET_BASE;
pub const DOT_I18N: &str = ".i18n";
pub const EMPTY: String = String::new();
pub const CONF_YML: &str = "conf.yml";

pub mod env;
use aok::Result;

// mod need_tran;
// use need_tran::need_tran;

mod find_i18n_dir;
pub use find_i18n_dir::{find_i18n_dir, find_i18n_dir_or_exit};

// mod lang_name;
// pub use lang_name::lang_name_li;
// mod tran_path;
// pub use tran_path::tran_path;
// mod tran_ext;
// pub use tran_ext::tran_ext;

pub mod conf;
pub use conf::Conf;

pub const HR: &str = "──────";
pub const NAME: &str = "i18";

const CACHE: &str = "cache";

pub fn token() -> String {
  if let Some(token) = env::token() {
    return token;
  }
  eprintln!(
    r#"
Please Set Token

1. get token from https://i18n.site/token

2. write `token: YOUR_TOKEN` in `~/.config/i18n.site.yml` or set env `I18N_SITE_TOKEN`
"#
  );
  std::process::exit(1);
}

pub fn i18n_conf_path(dir: &Path) -> PathBuf {
  dir.join(DOT_I18N).join(CONF_YML)
}

pub async fn run(
  workdir: &std::path::Path,
  conf: &I18nConf,
  ignore: &GlobSet,
  token: &str,
) -> Result<bool> {
  match _run(workdir, conf, ignore, token).await {
    Ok(_) => {
      println!("✅ i18n.site translate");
      // if r.err_count == 0 {
      //   println!("✅ i18n.site translate");
      // } else {
      //   println!(
      //     "\ni18n.site translate total {} error {}",
      //     r.total_len, r.err_count
      //   );
      // }
      Ok(true)
    }
    Err(err) => {
      if let Some(e) = err.downcast_ref::<crate::Err>() {
        if let crate::Err::Api { code, msg } = e {
          use api::ErrCode;
          if let Ok::<ErrCode, _>(code) = (*code).try_into() {
            if code == ErrCode::PayOff {
              let msg: i18_json::PayOff = sonic_rs::from_str(msg)?;
              let asset: f64 = (msg.asset as f64) / 100.0;
              let cost: f64 = (msg.cost as f64) / COST_BASE;

              eprintln!(
                "\n---\n\n{}\n\nThis translation need ${cost}\nThe account asset is ${asset}",
                crate::i18n::TOPUP,
              );

              if msg.asset == 0 {
                eprintln!("{}", crate::i18n::TOPUP_INIT);
              }
              eprintln!("\n---\n");
            }
            return Ok(false);
          }
        }
        eprintln!("\n❌ {}", e);
        return Ok(false);
      }
      Err(err)
    }
  }
}

pub async fn _run(
  workdir: &std::path::Path,
  conf: &I18nConf,
  ignore: &GlobSet,
  token: &str,
) -> Null {
  let scan = Scan::new(workdir, conf, ignore);
  let i18n_gen = workdir.join(DOT_I18N);
  let cache = i18n_gen.join(CACHE);
  let cache: std::path::PathBuf = (&*cache.as_os_str().to_string_lossy()).into();
  // 写入 gitignore
  init_dir::ignore(&cache)?;

  let rel_li = scan.lang_rel_li_for_tran();
  let mut lm = len_mtime::LenMtime::load(cache, workdir)?;
  let mut i18_hash = i18_hash::I18Hash::new(workdir);

  let to_tran = i18_hash.changed(lm.is_change(rel_li)?)?;

  if to_tran.is_empty() {
    return OK;
  }

  let (lrs_li, path_li, update_cache_file_li) = prepare_li(to_tran, &scan);

  let body = crate::tzst::tzst(workdir, path_li, lrs_li, &scan.rel_ft)?;
  let id = ub64::b64e(xhash::xhash(&body));

  let r = tran(token, &id, body).await?;
  let traning = print_tran_result(r).await?;

  // 会在save创建的时候, 更新译文修改的缓存时间和hash
  let mut save = Save::new(
    workdir,
    lm,
    i18_hash,
    scan.rel_ft,
    update_cache_file_li,
    &traning.update_cache,
  );

  let has_traned = !traning.traned.is_empty();
  if has_traned || !traning.end {
    if has_traned {
      println!("\n# Cached");
      for (rel, TranedLi { li }) in &traning.traned {
        for i in li {
          println!("{}/{rel}", LANG_CODE[i.lang as usize]);
        }
      }
      println!("{}", HR);
      save.save(&traning.traned)?;
    }
    if !traning.end {
      wait_tran(&id, save).await?;
      return OK;
    }
  }
  if traning.end {
    println!(
      "COST $0\nREMAIN ASSET ${}",
      (traning.asset as f64) / ASSET_BASE
    );
  }
  OK
}
