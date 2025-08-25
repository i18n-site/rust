use std::{collections::BTreeMap, future::Future, sync::Arc};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use faststr::FastStr;
use parking_lot::RwLock;
use rustls_pemfile::{certs, private_key};
use tokio::time;
use x509_parser::{extensions::GeneralName, parse_x509_certificate};

use crate::{
  cert::Cert,
  error::{Error, Result},
};

pub trait CertStrDb: Send + Sync + 'static {
  fn get(
    &self,
    host: impl Into<FastStr>,
  ) -> impl Future<Output = Result<Option<(String, String)>>> + Send + Sync;
}

#[derive(Debug)]
pub struct CertLoader<D: CertStrDb> {
  pub cert: DashMap<FastStr, Arc<Cert>>,
  pub expire: RwLock<BTreeMap<u64, Vec<FastStr>>>,
  pub db: D,
}

impl<D: CertStrDb> CertLoader<D> {
  pub fn new(db: D) -> Arc<Self> {
    let s = Arc::new(Self {
      cert: DashMap::new(),
      expire: RwLock::new(BTreeMap::new()),
      db,
    });

    let s2 = s.clone();
    tokio::spawn(async move {
      let mut interval = time::interval(time::Duration::from_secs(24 * 60 * 60));
      loop {
        interval.tick().await;
        s2.rm_expired(3);
      }
    });

    s
  }

  pub fn rm_expired(&self, days_to_expire: u64) {
    let now_days = Utc::now().timestamp() as u64 / 86400;
    let mut to_rm_days = Vec::new();
    {
      for (expire_day, hosts) in self.expire.read().iter() {
        if *expire_day < now_days + days_to_expire {
          for host in hosts {
            self.cert.remove(host);
          }
          to_rm_days.push(*expire_day);
        } else {
          break;
        }
      }
    }

    {
      let mut expire_write = self.expire.write();
      for day in to_rm_days {
        expire_write.remove(&day);
      }
    }
  }

  pub fn get(
    &self,
    host: impl Into<FastStr>,
  ) -> impl std::future::Future<Output = Result<Option<Arc<Cert>>>> + Send + Sync {
    let host = host.into();

    async move {
      if let Some(cert) = self.cert.get(&host) {
        return Ok(Some(cert.clone()));
      }
      if let Some((cert_pem, key_pem)) = self.db.get(host.clone()).await? {
        let mut cert_reader = std::io::Cursor::new(cert_pem.as_bytes());
        let cert_chain: std::result::Result<Vec<_>, _> = certs(&mut cert_reader).collect();
        let cert_chain =
          cert_chain.map_err(|e| Error::CertParse(format!("{host} 证书解析失败: {e:?}")))?;

        for cert_der in &cert_chain {
          if let Ok((_, x509)) = parse_x509_certificate(cert_der.as_ref()) {
            let domains = if let Ok(Some(san)) = x509.subject_alternative_name() {
              san
                .value
                .general_names
                .iter()
                .filter_map(|name| match name {
                  GeneralName::DNSName(s) => Some(s.to_string()),
                  _ => None,
                })
                .collect::<Vec<String>>()
            } else {
              x509
                .subject()
                .iter_common_name()
                .filter_map(|cn| cn.attr_value().as_str().ok().map(|s| s.to_string()))
                .collect::<Vec<String>>()
            };

            if domains.is_empty() {
              continue;
            }

            let expire_ts = x509.validity().not_after.timestamp();
            let expire_day = expire_ts as u64 / 86400;
            self
              .expire
              .write()
              .entry(expire_day)
              .or_default()
              .push(host.clone());
            if let Some(_not_after) = DateTime::from_timestamp(expire_ts, 0) {}
          }
        }

        if cert_chain.is_empty() {
          return Err(Error::CertNotFound(format!(
            "exist certificate , but no cert for {host}"
          )));
        }

        // 解析私钥
        let mut key_reader = std::io::Cursor::new(key_pem.as_bytes());
        let key = private_key(&mut key_reader)
          .map_err(|e| Error::CertParse(format!("私钥解析失败: {e:?}")))?
          .ok_or(Error::PrivateKeyNotFound)?;

        let cert = Arc::new(Cert {
          rustls: crate::cert::RustlsCert {
            cert: cert_chain,
            key,
          },
          pem: crate::cert::PemCert {
            cert: cert_pem,
            key: key_pem,
          },
        });
        self.cert.insert(host, cert.clone());
        return Ok(Some(cert));
      }

      Ok(None)
    }
  }
}
