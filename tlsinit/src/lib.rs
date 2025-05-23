#![feature(const_trait_impl)]

use std::sync::Arc;

use rustls::{ClientConfig, RootCertStore};
use rustls_native_certs::load_native_certs;

#[static_init::dynamic]
pub static CLIENT: Arc<ClientConfig> = Arc::new({
  #[cfg(feature = "install_default")]
  rustls::crypto::ring::default_provider()
    .install_default()
    .expect("rustls provider install failed");

  let mut store = RootCertStore::empty();

  for cert in load_native_certs().unwrap() {
    store.add(cert).unwrap();
  }

  ClientConfig::builder()
    .with_root_certificates(store)
    .with_no_client_auth()
});
