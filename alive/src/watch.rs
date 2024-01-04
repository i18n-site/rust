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

// pub async fn watch<'a>(
//   kind: &'a Kind,
//   watch: &'a Watch,
//   host: &'a str,
//   kind_arg: &'a str,
//   watch_arg: &'a str,
//   task: impl Task,
// )

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
          let mut n = 0;
          for ip in ip_li.records() {
            if let Some(ip) = ip.data() {
              if let RData::$rec_type(ip) = ip {
                dbg!(std::net::IpAddr::$v(**ip));
                n += 1;
              }
            }
          }
          if n == 0 || true {
            errlog(
              kind,
              host,
              watch,
              format!("域名记录为空"),
              "https://atomgit.com/i18n-ops/conf/tree/main/dns",
            )
            .await?;
          }
        }
        Err(err) => {
          errlog(
            kind,
            host,
            watch,
            format!("域名解析失败 {err}"),
            "https://atomgit.com/i18n-ops/conf/tree/main/dns",
          )
          .await?;
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
