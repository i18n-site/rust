#![cfg_attr(docsrs, feature(doc_cfg))]

mod kv;
pub mod layout;

#[cfg(all(unix, feature = "systemd"))]
use std::env;

use kv::Kv;
use logforth::{append, filter::env_filter::EnvFilterBuilder};

#[static_init::dynamic]
pub static TZ: jiff::tz::TimeZone = jiff::tz::TimeZone::try_system().unwrap();

pub fn init() {
  // Check if we're in a systemd environment (unix systems with INVOCATION_ID)
  #[cfg(all(unix, feature = "systemd"))]
  if env::var("INVOCATION_ID").is_ok() {
    if let Ok(journald) = logforth_append_journald::Journald::new() {
      logforth::starter_log::builder()
        .dispatch(|d| {
          d.filter(EnvFilterBuilder::from_default_env().build())
            .append(journald)
        })
        .apply();
      return;
    }
    // If journald fails, fall back to stdout
  }

  // Fallback to stdout logging
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
    panic!("No logging backend available")
  }
}
