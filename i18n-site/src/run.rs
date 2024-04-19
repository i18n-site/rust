use std::{collections::HashMap, path::PathBuf};

use aok::{Result, OK};
use lang::{IntoEnumIterator, Lang, LANG_CODE, LANG_NAME};

use crate::{upload, Conf, Err, Site, Upload};

pub async fn run_conf<Up: Upload + Default>(
  channel: String,
  ver: String,
  dir: PathBuf,
  conf: Conf,
) -> Result<()> {
  // if let Some(from_to) = conf.i18n.fromTo {
  //   dbg!(from_to);
  // }

  let mut lang_path = HashMap::<usize, _>::new();

  let mut has_all = false;

  for (from_str, to_li) in conf.i18n.fromTo {
    if let Ok::<Lang, _>(from) = from_str.clone().try_into() {
      lang_path.insert(from as _, from_str);
      if to_li.is_empty() {
        has_all = true;
      } else {
        for istr in to_li.split(' ') {
          let istr = istr.to_owned();
          if let Ok::<Lang, _>(i) = istr.clone().try_into() {
            lang_path.insert(i as usize, istr);
          }
        }
      }
    }
  }

  if has_all {
    for i in Lang::iter() {
      let i = i as usize;
      lang_path.entry(i).or_insert_with(|| LANG_CODE[i].into());
    }
  }

  let lang_path = lang_path
    .into_iter()
    .map(|(code, en)| (code as u32, en, LANG_NAME[code]))
    .collect::<Vec<_>>();

  let mut nav_code_li = Vec::with_capacity(conf.nav.len());
  let nav_li = conf
    .nav
    .into_iter()
    .map(|code| {
      let (code, url) = if let Some(pos) = code.rfind(' ') {
        (code[..pos].into(), code[pos + 1..].into())
      } else {
        (code.clone(), code)
      };
      nav_code_li.push(code);
      url
    })
    .collect::<Vec<_>>();

  Up::default()
    .run(
      Site {
        host: conf.host,
        channel,
        ver,
        route_li: conf.route,
        nav_li,
      },
      dir,
      lang_path,
      conf.upload.ext,
      nav_code_li,
    )
    .await?;

  println!("✅");
  OK
}

pub async fn run(channel: String, ver: String, dir: PathBuf, upload_s3: bool) -> Result<()> {
  let conf_fp = dir.join("conf.yml");
  let conf = ifs::r(&conf_fp)?;

  match serde_yaml::from_slice::<Conf>(&conf[..]) {
    Ok(conf) => {
      macro_rules! run_conf {
        ($up:ty) => {
          run_conf::<$up>(channel, ver, dir, conf).await
        };
      }
      if upload_s3 {
        run_conf!(upload::S3)
      } else {
        run_conf!(upload::Fs)
      }
    }
    Err(e) => Err(Err::Conf(conf_fp, e).into()),
  }
}
