use aok::{Result, OK};
use enum_dispatch::enum_dispatch;
use hickory_proto::rr::record_type::RecordType;
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
  ) -> Result<()>;
}

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

  match dns_type {
    4 | 6 => {
      match RESOLVER
        .lookup(
          host,
          if watch.dns_type == 6 {
            RecordType::AAAA
          } else {
            RecordType::A
          },
        )
        .await
      {
        Ok(ip_li) => {
          hi::send(
            "DNS_RESOLVER_ERROR",
            format!("{host} IPV{dns_type} {kind_v} watch_id={watch_id}"),
            "",
          )
          .await;
          // todo 添加超时, 用 try join
          match task.ping(kind, watch, host, kind_arg, watch_arg).await {
            Ok(_) => {
              // ok(kind, watch)
            }
            Err(err) => todo!(),
          }
        }
        Err(err) => {
          hi::send(
            "DNS_RESOLVER_ERROR",
            format!("{host} IPV{dns_type} {kind_v} watch_id={watch_id}\n{err}"),
            "",
          )
          .await;
        }
      }
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
