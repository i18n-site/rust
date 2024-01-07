pub use anyhow;

#[macro_export]
macro_rules! ok_or {
  ($expr:expr,$default:expr) => {{ (move || Ok::<_, $crate::anyhow::Error>($expr))() }.unwrap_or($default)};
}
