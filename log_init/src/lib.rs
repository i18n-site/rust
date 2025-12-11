#![cfg_attr(docsrs, feature(doc_cfg))]

mod kv;
pub mod layout;

use kv::Kv;
use logforth::{append, filter::env_filter::EnvFilterBuilder};

#[static_init::dynamic]
pub static TZ: jiff::tz::TimeZone = jiff::tz::TimeZone::try_system().unwrap();

pub fn init() {
  #[cfg(feature = "stdout")]
  {
    let stdout = append::Stdout::default().with_layout(layout::Text::default());

    logforth::starter_log::builder()
      .dispatch(|d| {
        d.filter(EnvFilterBuilder::from_default_env().build())
          .append(stdout)
      })
      .apply();
  }
  #[cfg(not(feature = "stdout"))]
  {
    panic!("No stdout feature enabled")
  }
}
