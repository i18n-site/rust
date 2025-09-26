#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use aok::{OK, Void};
use clap_args::arg;
use proxy_fetch::load;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .unwrap();
}

genv::def!(PROXY_SUBSCRITION_URL, PROXY_USER, PROXY_PASSWORD);

#[tokio::main]
async fn main() -> Void {
  if let Some(matches) = clap_args::parse!(|cmd| {
    cmd.arg(arg!(-b --bind [BIND] "http proxy bind address").default_value("0.0.0.0:15080"))
  }) {
    let bind_addr = matches.get_one::<String>("bind").unwrap();
    println!(
      "{} v{} {bind_addr}",
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_VERSION"),
    );
    let fetch = load(PROXY_SUBSCRITION_URL::<String>().split(";")).await?;
    let addr = bind_addr.parse()?;
    proxy_http::run(
      fetch,
      addr,
      PROXY_USER::<String>(),
      PROXY_PASSWORD::<String>(),
    )
    .await?;
  }
  OK
}
