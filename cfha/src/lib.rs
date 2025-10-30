pub mod conf;

use aok::Result;
pub use cfapi::endpoints::dns::DnsContent;
use cfapi::{
  endpoints::{
    dns::{
      CreateDnsRecord, CreateDnsRecordParams, DeleteDnsRecord, DeleteDnsRecordResponse, DnsRecord,
      ListDnsRecords, ListDnsRecordsParams,
    },
    zone::{ListZones, ListZonesParams, Zone},
  },
  framework::{Environment, HttpApiClientConfig, async_api::Client, auth::Credentials},
};

pub struct Cfha {
  api: Client,
}

macro_rules! is_ok {
  ($r:expr) => {{
    let r = $r;
    if !r.errors.is_empty() {
      return Err(Error::Api(r.errors).into());
    }
    r.result
  }};
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{0:?}")]
  Api(Vec<cfapi::framework::response::ApiError>),
}

impl Cfha {
  pub fn new(api_key: impl Into<String>, email: impl Into<String>) -> Result<Self> {
    let credentials = Credentials::UserAuthKey {
      key: api_key.into(),
      email: email.into(),
    };
    let config = HttpApiClientConfig::default();
    let api = Client::new(credentials, config, Environment::Production)?;
    Ok(Cfha { api })
  }

  pub async fn zone(&self, host: &str) -> Result<Option<Zone>> {
    let list_zones_params = ListZones {
      params: ListZonesParams {
        name: Some(host.to_string()),
        ..Default::default()
      },
    };
    let zones_response = is_ok!(self.api.request(&list_zones_params).await?);

    Ok(zones_response.into_iter().find(|z| z.name == host))
  }

  pub async fn record(
    &self,
    zone_id: impl AsRef<str>,
    host: impl AsRef<str>,
  ) -> Result<Vec<DnsRecord>> {
    let zone_id = zone_id.as_ref();
    let host = host.as_ref();
    let dns_records = is_ok!(
      self
        .api
        .request(&ListDnsRecords {
          zone_identifier: zone_id,
          params: ListDnsRecordsParams {
            name: Some(host.to_string()),
            ..Default::default()
          },
        })
        .await?
    );

    Ok(dns_records)
  }

  pub async fn rm_record(
    &self,
    zone_id: impl AsRef<str>,
    record_id: impl AsRef<str>,
  ) -> Result<DeleteDnsRecordResponse> {
    let zone_id = zone_id.as_ref();
    let record_id = record_id.as_ref();
    // 删除特定的 DNS 记录
    let delete_dns_record = DeleteDnsRecord {
      zone_identifier: zone_id,
      identifier: record_id,
    };

    let r = is_ok!(self.api.request(&delete_dns_record).await?);
    Ok(r)
  }
  pub async fn add_a_record(
    &self,
    zone_id: impl AsRef<str>,
    name: impl AsRef<str>,
    content: DnsContent,
    proxied: bool,
  ) -> Result<DnsRecord> {
    let zone_id = zone_id.as_ref();
    let create_dns_record = CreateDnsRecord {
      zone_identifier: zone_id,
      params: CreateDnsRecordParams {
        proxied: Some(proxied),
        name: name.as_ref(),
        ttl: None,
        priority: None,
        content,
      },
    };

    Ok(is_ok!(self.api.request(&create_dns_record).await?))
  }
}
