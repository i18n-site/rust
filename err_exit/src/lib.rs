#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_export]
macro_rules! err_exit {
  ($($arg:tt)*) => {{
    eprintln!(
      "❌ {}",
      format!($($arg)*)
    );
    std::process::exit(1)
  }};
}
