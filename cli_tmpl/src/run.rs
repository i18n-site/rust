use std::path::PathBuf;

use aok::{Result, OK};

pub async fn run(workdir: PathBuf) -> Result<()> {
  tracing::info!("{:?}", workdir);
  // let conf_fp = workdir.join("conf.yml");
  // let conf = ifs::r(&conf_fp)?;
  //
  // match serde_yaml::from_slice::<Conf>(&conf[..]) {
  //   Ok(conf) => {
  //     // if let Some(from_to) = conf.i18n.fromTo {
  //     // }
  //   }
  //   Err(e) => {
  //     return Err(crate::Err::Conf(conf_fp, e).into());
  //   }
  // }

  OK
}
