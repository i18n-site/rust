use std::time::Duration;

pub use indicatif::*;

pub fn pbar(total: u64) -> ProgressBar {
  let pb = pbar_no_run(total);
  pb.enable_steady_tick(Duration::from_millis(100));
  pb
}

pub fn pbar_no_run(total: u64) -> ProgressBar {
  let pb = ProgressBar::new(total);
  pb.set_style(
    ProgressStyle::default_bar()
      .template(
        "{msg} {wide_bar:.green/gray} {percent}% {spinner:.yellow} {elapsed_precise} ETA {eta}",
      )
      .unwrap()
      .progress_chars("=>─"),
  );
  pb
}
