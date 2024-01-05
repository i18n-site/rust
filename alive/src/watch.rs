use std::{net::IpAddr, time::Duration};

use aok::{Result, OK};
use enum_dispatch::enum_dispatch;
use futures::{stream::FuturesOrdered, StreamExt};
use hickory_proto::rr::{record_type::RecordType, RData};
use xstr::Join;

use crate::{
  db::{Kind, Watch},
  dberr, errlog, ok,
};

#[enum_dispatch]
pub trait Task {
  async fn ping<'a>(
    &self,
    kind: &'a Kind,
    watch: &'a Watch,
    host: &'a str,
    kind_arg: &'a str,
    watch_arg: &'a str,
    ip: IpAddr,
  ) -> Result<()>;
}

pub const DNS_URL: &str = "https://atomgit.com/i18n-ops/conf/tree/main/dns";

pub async fn _watch<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
  task: impl Task,
) -> Result<()> {
  let dns_type = watch.dns_type;
  let watch_id = watch.id;

  macro_rules! dns {
    ($rec_type:ident, $v:ident) => {
      match idns::lookup(host, RecordType::$rec_type).await {
        Ok(ip_li) => {
          let mut addr_li = Vec::new();
          for ip in ip_li.records() {
            if let Some(RData::$rec_type(ip)) = ip.data() {
              addr_li.push(std::net::IpAddr::$v(**ip));
            }
          }
          if addr_li.is_empty() {
            errlog(kind, host, watch, format!("域名记录为空"), DNS_URL).await?;
          } else {
            let mut ing = FuturesOrdered::from_iter(
              addr_li
                .iter()
                .map(|addr| task.ping(kind, watch, host, kind_arg, watch_arg, *addr)),
            );

            let mut n = 0;
            let mut failed_addr = Vec::new();
            while let Some(result) = ing.next().await {
              if let Err(err) = result {
                failed_addr.push(format!("IP地址 {}\n{err}", addr_li[n]));
              }
              n += 1;
            }

            if !failed_addr.is_empty() {
              let len = failed_addr.len();
              let txt = format!(
                "{}\n域名解析总地址数 {}  / 出错的IP地址数 {}",
                failed_addr.join("\n\n"),
                addr_li.len(),
                len,
              );

              errlog(kind, host, watch, txt, "").await?;
            } else {
              ok(
                kind,
                watch,
                host,
                || "IP地址:\n".to_owned() + &addr_li.join("\n"),
                "",
              )
              .await?;
            };
          }
        }
        Err(err) => {
          errlog(kind, host, watch, format!("域名解析失败 {err}"), DNS_URL).await?;
        }
      }
    };
  }

  match dns_type {
    4 => {
      dns!(A, V4)
    }
    6 => {
      dns!(AAAA, V6)
    }
    _ => {
      dberr!(
        DnsTypeNotSupported
        "dns_type={} watch_id={} {}",
        dns_type,watch_id,host
      );
      return OK;
    }
  }
  OK
}

pub const TIMEOUT: Duration = Duration::from_secs(180);

pub async fn watch<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
  task: impl Task,
) -> Result<()> {
  match tokio::time::timeout(
    TIMEOUT,
    _watch(kind, watch, host, kind_arg, watch_arg, task),
  )
  .await
  {
    Err(_) => {
      errlog(
        kind,
        host,
        watch,
        format!("运行超时 ( {} 分钟 )", TIMEOUT.as_secs() / 60),
        "",
      )
      .await?;
      OK
    }
    Ok(r) => r,
  }
}
