#![feature(coroutines, coroutine_trait)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
#![feature(effects)]

use globset::GlobSet;
use pbar::pbar;
mod txn;
pub use txn::Txn;

mod err;
pub use err::Err;

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub const DOT_I18N: &str = ".i18n";
pub const EMPTY: String = String::new();
pub const CONF_YML: &str = "conf.yml";

pub mod env;
mod need_tran;
use aok::Result;
use need_tran::need_tran;

mod lang_name;
pub use lang_name::lang_name_li;
mod tran_path;
pub use tran_path::tran_path;
mod tran_ext;
pub use tran_ext::tran_ext;

pub mod conf;
pub use conf::{build_ignore, Conf, I18nConf};

pub const NAME: &str = "i18";

const CACHE: &str = "cache";

pub fn token() -> String {
  if let Some(token) = env::token() {
    return token;
  }
  eprintln!(
    r#"
Please Set Token

1. found token in https://i18n.site/token

2. set env 'I18N_SITE_TOKEN' or config `token: your_token` in ~/.config/i18n.site.yml
"#
  );
  std::process::exit(1);
}

pub fn i18n_conf<Conf: serde::de::DeserializeOwned>(workdir: &std::path::Path) -> Result<Conf> {
  let fp = workdir.join(DOT_I18N).join(CONF_YML);
  let conf = ifs::r(&fp)?;
  match serde_yaml::from_slice(&conf) {
    Ok(conf) => Ok(conf),
    Err(err) => Err(crate::err::Err::Conf(fp, err).into()),
  }
}

pub fn conf(workdir: &std::path::Path) -> Result<Conf> {
  let conf: Conf = i18n_conf(workdir)?;
  Ok(conf)
}

pub async fn run(
  workdir: &std::path::Path,
  conf: &I18nConf,
  ignore: &GlobSet,
  token: String,
) -> Result<usize> {
  let i18n_gen = workdir.join(DOT_I18N);

  let cache = i18n_gen.join(CACHE);
  let cache: std::path::PathBuf = (&*cache.as_os_str().to_string_lossy()).into();

  #[allow(clippy::never_loop)]
  loop {
    if let Ok(meta) = std::fs::metadata(&cache) {
      if meta.is_dir() {
        break;
      }
      std::fs::remove_file(&cache)?;
    }
    std::fs::create_dir_all(&cache)?;

    use std::io::Write;
    ifs::w(i18n_gen.join(".gitignore"))?.write_all(format!("/{}/", CACHE).as_bytes())?;
    break;
  }

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
    let db = redb::Database::create(cache.join(stringify!($ext)))?;

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
  //token.as_ref(),
  let all_li = ext!(md, yml);

  let mut total_len = 0;
  for (li, ..) in &all_li {
    for i in li {
      total_len += i.len;
    }
  }

  let mut err_count = 0;

  if total_len > 0 {
    let pb = pbar(total_len);

    for (li, db, ext, from_to) in all_li {
      let from_to = from_to.into();
      for i in li {
        let rel = i.rel.clone();
        let len = i.len;
        pb.set_message(rel.clone());
        if let Err(err) = tran_path(i, &token, &db, ext, workdir, &from_to).await {
          err_count += 1;
          eprintln!("❌ {rel} : {:?}", err);
        }

        pb.inc(len);
      }
    }
    pb.finish();
  }

  println!("\n✅ i18n.site translate");
  Ok(err_count)
}
