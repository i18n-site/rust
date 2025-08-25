use rustls_pki_types::{CertificateDer, PrivateKeyDer};

// ecc 证书
#[derive(Debug)]
pub struct RustlsCert {
  pub cert: Vec<CertificateDer<'static>>,
  pub key: PrivateKeyDer<'static>,
}

#[derive(Debug)]
pub struct PemCert {
  pub cert: String,
  pub key: String,
}

#[derive(Debug)]
pub struct Cert {
  pub rustls: RustlsCert,
  pub pem: PemCert,
}
