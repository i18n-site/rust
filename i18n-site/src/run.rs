// use std::{collections::HashMap, path::PathBuf};
use std::{io::Write, path::PathBuf};

use aok::{ensure, Null, OK};

use crate::{gen, Conf, VDir};
// use lang::{IntoEnumIterator, Lang, LANG_CODE, LANG_NAME};
//
// use crate::{
//   upload::{upload, Fs, Payload, S3},
//   Conf, Err, Site, Upload,
// };

// pub async fn run_conf(channel: String, ver: String, dir: PathBuf, conf: Conf) -> Result<Payload> {
//   // if let Some(from_to) = conf.i18n.fromTo {
//   // }
//
//   let config: i18::Config = i18::Conf {
//     i18n: conf.i18n,
//     ignore: conf.ignore,
//   }
//   .into();
//
//   i18::run(&dir, &config, i18::token()).await?;
//   let mut lang_path = HashMap::<usize, _>::new();
//
//   let mut has_all = false;
//
//   for (from_str, to_li) in config.i18n.fromTo {
//     if let Ok::<Lang, _>(from) = from_str.clone().try_into() {
//       lang_path.insert(from as _, from_str);
//       if to_li.is_empty() {
//         has_all = true;
//       } else {
//         for istr in to_li.split(' ') {
//           let istr = istr.to_owned();
//           if let Ok::<Lang, _>(i) = istr.clone().try_into() {
//             lang_path.insert(i as usize, istr);
//           }
//         }
//       }
//     }
//   }
//
//   if has_all {
//     for i in Lang::iter() {
//       let i = i as usize;
//       lang_path.entry(i).or_insert_with(|| LANG_CODE[i].into());
//     }
//   }
//
//   let lang_path = lang_path
//     .into_iter()
//     .map(|(code, en)| (code as u32, en, LANG_NAME[code]))
//     .collect::<Vec<_>>();
//
//   let mut nav_code_li = Vec::with_capacity(conf.nav.len());
//   let nav_li = conf
//     .nav
//     .into_iter()
//     .map(|code| {
//       let (code, url) = if let Some(pos) = code.rfind(' ') {
//         let url = &code[pos + 1..];
//         (
//           code[..pos].into(),
//           (if url == "/" { "" } else { url }).into(),
//         )
//       } else {
//         (code.clone(), code)
//       };
//       nav_code_li.push(code);
//       url
//     })
//     .collect::<Vec<_>>();
//
//   upload(
//     Site {
//       host: conf.host,
//       channel,
//       ver,
//       route_li: conf.route,
//       nav_li,
//     },
//     dir,
//     lang_path,
//     conf.upload.ext,
//     &config.ignore,
//     nav_code_li,
//   )
//   .await
// }
//

pub async fn run(dir: PathBuf, channel_ver: Option<String>, upload_s3: bool) -> Null {
  use chrono::Local;

  let (channel, ver) = if let Some(channel_ver) = channel_ver {
    let channel_ver: Vec<_> = channel_ver.split(':').collect();
    ensure!(channel_ver.len() == 2);
    (channel_ver[0].into(), channel_ver[1].into())
  } else {
    (
      "nightly".to_owned(),
      Local::now().format("%Y-%m-%d").to_string(),
    )
  };

  ifs::w(dir.join("v/@/@"))?.write_all(format!("{}:{}", channel, ver).as_bytes())?;
  let vdir = VDir::new(&dir);

  let conf_fp = dir.join("conf.yml");
  let conf = ifs::r(&conf_fp)?;

  match serde_yaml::from_slice::<Conf>(&conf[..]) {
    Ok(conf) => {
      // dbg!(vdir.find("doc/x/y/z"));
      // dbg!(vdir.find("doc/xyz"));
      // dbg!(vdir.find("xdoc/xyz"));
      // let payload = run_conf(channel, ver, dir, conf).await?;
      // Fs.upload(&payload).await?;
      // if upload_s3 {
      //   S3::default().upload(&payload).await?;
      // };

      gen(dir, conf, vdir, upload_s3).await?;

      println!("✅ i18n.site build");
      OK
    }
    Err(e) => Err(crate::Err::Conf(conf_fp, e).into()),
  }
  // let ver = m
  //   .get_one("dist_ver")
  //   .map(|s: &String| s.into())
  //   .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
  //   let conf_fp = dir.join("conf.yml");
  //   let conf = ifs::r(&conf_fp)?;
  //
  //   match serde_yaml::from_slice::<Conf>(&conf[..]) {
  //     Ok(conf) => {
  //       let payload = run_conf(channel, ver, dir, conf).await?;
  //       Fs.upload(&payload).await?;
  //       if upload_s3 {
  //         S3::default().upload(&payload).await?;
  //       };
  //       println!("✅ i18n.site build");
  //       OK
  //     }
  //     Err(e) => Err(Err::Conf(conf_fp, e).into()),
  //   }
}
