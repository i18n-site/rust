use std::path::PathBuf;

use aok::{Result, OK};

use crate::Conf;

pub async fn run(workdir: PathBuf) -> Result<()> {
  let conf_fp = workdir.join("conf.yml");
  let conf = ifs::r(&conf_fp)?;

  match serde_yaml::from_slice::<Conf>(&conf[..]) {
    Ok(conf) => {
      dbg!(&conf);
      // if let Some(from_to) = conf.i18n.fromTo {
      //   dbg!(from_to);
      // }
    }
    Err(e) => {
      return Err(crate::Err::Conf(conf_fp, e).into());
    }
  }

  OK
}
