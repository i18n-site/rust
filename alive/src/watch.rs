use std::net::IpAddr;

use aok::{Result, OK};
use enum_dispatch::enum_dispatch;
use futures::{stream::FuturesOrdered, StreamExt};
use hickory_proto::rr::{record_type::RecordType, RData};
use hickory_resolver::{
  config::{ResolverConfig, ResolverOpts},
  TokioAsyncResolver,
};

use crate::{
  db::{Kind, Watch},
  dberr, errlog, ok,
};

#[static_init::dynamic]
pub static RESOLVER: TokioAsyncResolver =
  TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());

#[enum_dispatch]
pub trait Task {
  async fn ping<'a>(
    &self,
    kind: &'a Kind,
    watch: &'a Watch,
    host: &'a str,
    kind_arg: &'a str,
    watch_arg: &'a str,
    addr: IpAddr,
  ) -> Result<()>;
}

pub const DNS_URL: &'static str = "https://atomgit.com/i18n-ops/conf/tree/main/dns";

pub async fn watch<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
  task: impl Task,
) -> Result<()> {
  let dns_type = watch.dns_type;
  let watch_id = watch.id;
  let kind_v = &kind.v;
  // todo 添加超时, 用 try join
  // match task.ping(kind, watch, host, kind_arg, watch_arg).await {
  //   Ok(_) => {
  //     // ok(kind, watch)
  //   }
  //   Err(err) => {
  //     dbg!("todo");
  //   }
  // }

  macro_rules! dns {
    ($rec_type:ident, $v:ident) => {
      match RESOLVER.lookup(host, RecordType::$rec_type).await {
        Ok(ip_li) => {
          let mut addr_li = Vec::new();
          for ip in ip_li.records() {
            if let Some(ip) = ip.data() {
              if let RData::$rec_type(ip) = ip {
                addr_li.push(std::net::IpAddr::$v(**ip));
              }
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
              let txt = failed_addr.join("\n\n");
              errlog(kind, host, watch, txt, "").await?;
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
