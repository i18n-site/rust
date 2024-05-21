// use std::{collections::HashMap, path::PathBuf};
use std::path::PathBuf;

use aok::{Null, OK};
use i18::{i18n_conf, DOT_I18N};

use crate::{gen, yml_li_lpush, Conf, VDir};

pub async fn run(dir: PathBuf, mut ver: Option<String>, upload_s3: bool) -> Null {
  let vdir = dir.join(DOT_I18N).join("v");
  let v_yml = vdir.join("v.yml");
  if ver.is_none() && !v_yml.exists() {
    ver = Some("0.0.1".into());
  }

  if let Some(ver) = ver {
    yml_li_lpush(&vdir.join("v.yml"), &ver)?;
  }

  let vdir = VDir::new(&vdir);

  let conf = i18n_conf::<Conf>(&dir)?;
  // let payload = run_conf(channel, ver, dir, conf).await?;
  // Fs.upload(&payload).await?;
  // if upload_s3 {
  //   S3::default().upload(&payload).await?;
  // };

  gen(&dir, conf, vdir, upload_s3).await?;

  println!("✅ i18n.site build");
  OK
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
