#![feature(coroutines, coroutine_trait)]
#![feature(let_chains)]

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub mod env;
use aok::{Result, OK};
use clap::arg;
use cmdv::cmdv;
use static_init::constructor;

mod lang_name;
pub use lang_name::lang_name_li;
mod tran_path;
pub use tran_path::tran_path;
mod tran_ext;
pub use tran_ext::tran_ext;
mod from_to;
pub use from_to::FromTo;
// mod watch;
// pub use watch::{Change, Watch};
// mod tran_parent;

mod conf;
pub use conf::Conf;
pub mod mirror;
// mod tran_dir;
// pub use tran_dir::tran_dir;

pub const NAME: &str = "i18";

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// pub async fn tran_by_conf(
//   conf: &Conf,
//   from_to: &HashMap<String, String>,
//   dir: &PathBuf,
// ) -> Result<()> {
//   let mut to_from = HashMap::new();
//   let mut from_to = HashMap::new();
//   let mut default_from_lang = None;
//
//   let mut exist = HashSet::new();
//
//   for (kstr, v) in &conf.fromTo {
//     let kstr = kstr.to_owned();
//     if let Ok::<Lang, _>(klang) = kstr.as_str().try_into() {
//       exist.insert(klang);
//       let k = LangStr {
//         lang: klang,
//         str: kstr,
//       };
//       if v.is_empty() {
//         default_from_lang = Some(k);
//       } else {
//         let v = v.split(' ').collect::<Vec<_>>();
//         let mut li = Vec::with_capacity(v.len());
//         for i in v {
//           if let Ok::<Lang, _>(lang) = i.try_into() {
//             if lang == klang {
//               continue;
//             }
//             exist.insert(lang);
//             let lang_str = LangStr {
//               lang,
//               str: i.into(),
//             };
//             li.push(lang_str.clone());
//             to_from.insert(lang_str, k.clone());
//           }
//         }
//         from_to.insert(k, li);
//       }
//     }
//   }
//
//   if let Some(default_from_lang) = default_from_lang {
//     let mut li = Vec::with_capacity(CODE_LANG.len());
//     for i in &CODE_LANG {
//       if !exist.contains(i.1) {
//         li.push(LangStr {
//           lang: *i.1,
//           str: (*i.0).into(),
//         })
//       }
//     }
//     from_to.insert(default_from_lang, li);
//   }
//
//   let mut watch = Watch::new(dir);
//
//   let mut traned_from = HashSet::new();
//
//   for (k, to_lang_li) in &from_to {
//     tran_parent(
//       &mut traned_from,
//       &from_to,
//       &to_from,
//       dir,
//       k,
//       to_lang_li,
//       &mut watch,
//     )
//     .await?;
//   }
//
//   watch.purge();
//   OK
// }

const CACHE: &str = "cache";

pub async fn run() -> Result<()> {
  if let Some(cmd) = cmdv!(i18) {
    let cmd = cmd.arg(arg!(-d --workdir [path] "workdir"));
    let m = cmd.get_matches();
    let workdir = m
      .get_one("workdir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    let gen = workdir.join(".gen");

    let cache = gen.join(CACHE);
    let cache: std::path::PathBuf = (&*cache.as_os_str().to_string_lossy()).into();

    #[allow(clippy::never_loop)]
    loop {
      if let Ok(meta) = std::fs::metadata(&cache) {
        if meta.is_file() {
          break;
        }
        std::fs::remove_dir_all(&cache)?;
      }

      use std::io::Write;
      ifs::w(gen.join(".gitignore"))?.write_all(CACHE.as_bytes())?;
      break;
    }

    let db = redb::Database::create(cache)?;

    let conf = workdir.join("conf.yml");
    let conf = ifs::r(conf)?;
    let conf: Conf = serde_yaml::from_slice(&conf)?;

    if let Some(token) = env::token() {
      macro_rules! ext {
        ($($ext:ident),*) => {
          $(
            let $ext = if let Some($ext) = &conf.$ext
              && let Some(ft) = &$ext.fromTo
            {
              ft
            } else {
              &conf.fromTo
            };
          )*
          tokio::try_join!(
            $(
              tran_ext(
                token.as_ref(),
                &workdir,
                $ext,
                stringify!($ext),
                &db,
              )
            ),*
          )?;
        };
      }
      ext!(md, yml);
    } else {
      eprintln!(
        r#"
Please Set Token

1. found token in https://i18n.site/token

2. set env 'I18N_SITE_TOKEN' or config `token: your_token` in ~/.config/i18n.site.yml
"#
      );
      std::process::exit(1);
    }
  }

  OK
}
