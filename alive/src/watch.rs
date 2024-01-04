use aok::{Result, OK};
use enum_dispatch::enum_dispatch;
use hickory_resolver::{
  config::{ResolverConfig, ResolverOpts},
  Resolver,
};

use crate::{
  db::{Kind, Watch},
  dberr, errlog, ok,
};

#[static_init::dynamic]
pub static RESOLVER: Resolver =
  Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

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
  match watch.dns_type {
    4 => {}
    6 => {}
    _ => {
      dberr!(
        DnsTypeNotSupported
        "watch_id={} host={} dns_type={}",
        watch.dns_type,
        host,
        watch.id
      );
      return OK;
    }
  }
  match task.ping(kind, watch, host, kind_arg, watch_arg).await {
    Ok(_) => {
      // ok(kind, watch)
    }
    Err(err) => todo!(),
  }
  OK
}
