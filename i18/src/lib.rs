#![feature(coroutines, coroutine_trait)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
use std::path::{Path, PathBuf};

pub use db::{Db, Table};
use globset::GlobSet;
use i18_conf::I18nConf;
use mreq::Mreq;
use pbar::pbar;
use prost::Message;
use static_init::dynamic;
pub mod i18n;
pub mod purge;

mod db;
mod err;
pub use err::Err;

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub const DOT_I18N: &str = ".i18n";
pub const EMPTY: String = String::new();
pub const CONF_YML: &str = "conf.yml";

#[dynamic]
pub static API: Vec<String> = {
  if let Ok(host_li) = std::env::var("API") {
    host_li.split(' ').map(|i| i.into()).collect()
  } else {
    vec![
      "s.i18n.site".into(),
      "c0.018007.xyz".into(),
      "a0.3ti.site".into(),
      "c1.018007.xyz".into(),
      "a1.3ti.site".into(),
      "c2.018007.xyz".into(),
    ]
  }
};

pub static API_TRAN: &str = "tran";
pub static API_TRAN_END: &str = "tranEnd";

pub mod env;
mod need_tran;
use aok::Result;
use need_tran::need_tran;

mod find_i18n_dir;
pub use find_i18n_dir::{find_i18n_dir, find_i18n_dir_or_exit};

mod lang_name;
pub use lang_name::lang_name_li;
mod tran_path;
pub use tran_path::tran_path;
mod tran_ext;
pub use tran_ext::tran_ext;

pub mod conf;
pub use conf::Conf;

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

pub struct RunResult {
  pub err_count: usize,
  pub total_len: u64,
}

pub async fn run(
  workdir: &std::path::Path,
  conf: &I18nConf,
  ignore: &GlobSet,
  token: &str,
) -> Result<bool> {
  match _run(workdir, conf, ignore, token).await {
    Ok(r) => {
      if r.err_count == 0 {
        println!("✅ i18n.site translate");
      } else {
        println!(
          "\ni18n.site translate total {} error {}",
          r.total_len, r.err_count
        );
      }
      Ok(true)
    }
    Err(err) => {
      if let Some(e) = err.downcast_ref::<crate::Err>() {
        if let crate::Err::Api(code) = e {
          if let Ok::<api::Err, _>(e) = (*code).try_into() {
            if e == api::Err::PayOff {
              eprintln!("\n---\n{}\n---\n", crate::i18n::TOPUP);
            } else {
              eprintln!("\n❌ {:?}", e);
            }
            return Ok(false);
          }
        };
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
) -> Result<RunResult> {
  let i18n_gen = workdir.join(DOT_I18N);

  let cache = i18n_gen.join(CACHE);
  let cache: std::path::PathBuf = (&*cache.as_os_str().to_string_lossy()).into();

  init_dir::ignore(&cache)?;

  let fjall = db::open(&cache)?;

  macro_rules! ext {
    ($($ext:ident),*) => {{
      [
$({
    let from_to = if let Some(i) = &conf.$ext
    {
      &i.fromTo
    } else {
      &conf.fromTo
    };

    let db = fjall.table(stringify!($ext))?;

    (
      tran_ext(
        &ignore,
        &workdir,
        from_to,
        stringify!($ext),
        &db,
      )?,
      db,
      stringify!($ext),
      from_to,
    )
}),*
      ]
    }}
  }

  let all_li = ext!(md, yml);

  let mut total_len = 0;
  for (li, ..) in &all_li {
    for i in li {
      total_len += i.len;
    }
  }

  let mut err_count = 0;

  if total_len > 0 {
    let mut char_sum = 0;
    let mut lang_count = 0;
    let mut all_n = 0;
    let mut req = Mreq::new(&API[..], [("t", token)]);
    let pb = pbar(total_len);

    for (li, db, ext, from_to) in all_li {
      let from_to = from_to.into();
      for i in li {
        let rel = i.rel.clone();
        let len = i.len;
        pb.set_message(rel.clone());
        match tran_path(i, &mut req, &db, ext, workdir, &from_to).await {
          Err(err) => {
            if let Some(e) = err.downcast_ref::<crate::Err>() {
              if e.is_exit() {
                return Err(err);
              }
            }
            err_count += 1;
            eprintln!("❌ {rel} : {}", err);
          }
          Ok((c, l)) => {
            all_n += 1;
            char_sum += c;
            lang_count += l;
          }
        }

        pb.inc(len);
      }
    }
    pb.finish();

    if char_sum > 0 {
      let lang_n = lang_count / all_n;
      let char_n = if lang_n > 0 {
        char_sum / lang_n
      } else {
        char_sum
      };
      let asset = req.post_no_body(API_TRAN_END).await?;
      let asset = api::Asset::decode(asset)?;
      let cost = (char_sum * asset.threshold_amount) as f64 / (asset.threshold as f64);
      println!(
        "CHAR {char_n} × LANG {lang_n} = COST ${:.3} ( Account Balance ${} )",
        cost / 100.0,
        asset.cent as f64 / 100.0
      );
    }
  }

  Ok(RunResult {
    err_count,
    total_len,
  })
}
