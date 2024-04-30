use std::{net::IpAddr, sync::Arc, time::Duration};

use aok::{ensure, throw, Null, OK};
use rustls::{pki_types::ServerName, ClientConfig, RootCertStore};
use rustls_native_certs::load_native_certs;
use tokio::{net::TcpStream, time};
use tokio_rustls::TlsConnector;
use x509_parser::prelude::{FromDer, X509Certificate};

#[static_init::dynamic]
pub static CLIENT_CONF: Arc<ClientConfig> = Arc::new({
  let mut store = RootCertStore::empty();
  for cert in load_native_certs().unwrap() {
    store.add(cert).unwrap();
  }

  ClientConfig::builder()
    .with_root_certificates(store)
    .with_no_client_auth()
});

pub async fn tls_ping(host: impl Into<String>, ip: IpAddr, timeout: u64) -> Null {
  let host = host.into();
  let addr = std::net::SocketAddr::new(ip, 443);

  let connector = TlsConnector::from(CLIENT_CONF.clone());
  let stream = TcpStream::connect(&addr).await?;
  let dns_name = ServerName::try_from(host.clone())?;
  let tls_stream = time::timeout(
    Duration::from_secs(timeout),
    connector.connect(dns_name, stream),
  )
  .await??;

  // let tls_stream = connector.connect(dns_name, stream).await?;

  if let Some(peer_certs) = tls_stream.get_ref().1.peer_certificates() {
    let mut has_host_cert = false;

    for cert in peer_certs {
      let cert = cert.to_vec();
      let cert = X509Certificate::from_der(&cert);

      for (_, i) in cert.iter() {
        let mut name = String::new();
        for i in i.subject().iter_common_name() {
          if !name.is_empty() {
            name += " ";
          }
          name += i.as_str()?;
        }

        if name == host || (name.starts_with("*.") && host == name[2..]) {
          has_host_cert = true;
          break;
        } else if let Some(h) = host.split_once('.').map(|x| x.1) {
          if h == name {
            has_host_cert = true;
            break;
          }
        }

        if let Some(expire_after) = i.validity.time_to_expiration() {
          let expire_after = (expire_after.as_seconds_f32() as u64) / 86400;
          ensure!(expire_after > 14, "{name} expire after {expire_after} days",);
        } else {
          throw!("{host} : {name} has not valid expire");
        }
      }
    }

    if !has_host_cert {
      throw!("{} no host cert", host);
    }
  } else {
    throw!("{} no cert", host);
  }
  OK
}
