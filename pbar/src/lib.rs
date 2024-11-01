use std::{borrow::Cow, ops::Deref, time::Duration};

use coarsetime::Clock;
pub use indicatif::*;

#[derive(Debug)]
pub struct Pbar {
  pub bar: ProgressBar,
  pub pre: u64,
}

impl Pbar {
  pub fn set_message(&mut self, msg: impl Into<Cow<'static, str>>) {
    let sec = Clock::now_since_epoch().as_secs();
    if sec - self.pre >= 1 {
      self.pre = sec;
      self.bar.set_message(msg);
    }
  }
}

impl Deref for Pbar {
  type Target = ProgressBar;

  fn deref(&self) -> &Self::Target {
    &self.bar
  }
}

pub fn pbar(total: u64) -> Pbar {
  let pb = pbar_no_run(total);
  pb.enable_steady_tick(Duration::from_millis(200));
  pb
}

pub fn pbar_no_run(total: u64) -> Pbar {
  let pb = ProgressBar::new(total);
  pb.set_style(
    ProgressStyle::default_bar()
      .template(
        "{msg} {wide_bar:.green/gray} {percent}% {spinner:.yellow} {elapsed_precise} ETA {eta}",
      )
      .unwrap()
      .progress_chars("=>─"),
  );
  Pbar { bar: pb, pre: 0 }
}
