pub use axum;
pub use const_str;

#[macro_export]
macro_rules! ping_ver {
  ($app:expr) => {{
    async fn ping_ver() -> &'static str {
      $crate::const_str::concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"))
    }
    $app.route("/ping/ver", $crate::axum::routing::get(ping_ver))
  }};
}
