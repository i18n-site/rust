use aok::{Result, OK};
use i18n_js::HtmConf;

pub async fn refresh(conf: &String) -> Result<()> {
  dbg!(conf);
  OK
}

pub async fn refresh_v(conf: &HtmConf) -> Result<()> {
  refresh(&conf.v).await?;
  // if let Some(i) = conf.importmap.get("i/") {
  //   refresh(i).await?;
  // }
  OK
}
