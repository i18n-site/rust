use core::net::Ipv4Addr;
use std::{net::IpAddr, time::Duration};

pub use hickory_proto;
use hickory_proto::rr::{record_type::RecordType, RData};
pub use hickory_resolver;
use hickory_resolver::{
  config::{NameServerConfigGroup, ResolverConfig, ResolverOpts},
  error::ResolveError,
  lookup::Lookup,
  IntoName, TokioAsyncResolver,
};

macro_rules! ipv4 {
  ($($i1:expr ; $i2:expr ; $i3:expr ; $i4:expr , )+) => {
    [
      $(IpAddr::V4(Ipv4Addr::new($i1,$i2,$i3,$i4))),+
    ]
  }
}
pub fn resolve_options() -> ResolverOpts {
  let mut opt = ResolverOpts::default();
  opt.attempts = 6;
  opt.num_concurrent_reqs = 3;
  opt.timeout = Duration::from_secs(6);
  opt
}

#[static_init::dynamic]
pub static RESOLVER_IPV4: TokioAsyncResolver = TokioAsyncResolver::tokio(
  ResolverConfig::from_parts(
    None,
    vec![],
    NameServerConfigGroup::from_ips_clear(
      // https://www.chengxulvtu.net/public-dns/
      &ipv4!(
        208;67;220;220,
        208;67;222;222,
        77;88;8;8,
        77;88;8;1,
        114;114;114;114,
        114;114;115;115,
        119;28;28;28,
        119;29;29;29,
        149;112;112;112,
        180;76;76;76,
        1;0;0;1,
        1;1;1;1,
        223;5;5;5,
        223;6;6;6,
        8;8;4;4,
        9;9;9;9,
        8;8;8;8,
      ),
      53,
      true,
    ),
  ),
  resolve_options(),
);

#[static_init::dynamic]
pub static RESOLVER_TLS: TokioAsyncResolver = {
  let mut conf = ResolverConfig::default();
  for dns in [
    NameServerConfigGroup::from_ips_https(
      &[
        IpAddr::V4([223, 6, 6, 6].into()),
        IpAddr::V4([223, 5, 5, 5].into()),
      ],
      443,
      "dns.alidns.com".into(),
      true,
    ),
    NameServerConfigGroup::from_ips_https(
      &[
        IpAddr::V4([120, 53, 53, 53].into()),
        IpAddr::V4([1, 12, 12, 12].into()),
      ],
      443,
      "doh.pub".into(),
      true,
    ),
    NameServerConfigGroup::google_https(),
    NameServerConfigGroup::cloudflare_https(),
    NameServerConfigGroup::quad9_https(),
  ] {
    for i in dns.iter() {
      conf.add_name_server(i.clone());
    }
  }
  TokioAsyncResolver::tokio(conf, resolve_options())
};

pub async fn lookup<N: IntoName>(name: N, record_type: RecordType) -> Result<Lookup, ResolveError> {
  let name = name.into_name()?;
  match RESOLVER_TLS.lookup(name.clone(), record_type).await {
    Ok(r) => Ok(r),
    Err(err) => {
      tracing::warn!("DNS ERROR: {}", err);
      RESOLVER_IPV4.lookup(name, record_type).await
    }
  }
}

macro_rules! ip_li {
  ($host:ident,$type:ident, $ver:ident) => {{
    let li = lookup($host, RecordType::$type).await?;
    let li = li.records();
    let mut r = Vec::with_capacity(li.len());
    for i in li {
      if let Some(RData::$type(ip)) = i.data() {
        r.push(std::net::IpAddr::$ver(**ip));
      }
    }
    Ok(r)
  }};
}

#[allow(non_snake_case)]
pub async fn A<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  ip_li!(name, A, V4)
}

#[allow(non_snake_case)]
pub async fn AAAA<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  ip_li!(name, AAAA, V6)
}

static mut USE_IPV6: bool = true;

pub async fn ip<N: IntoName>(name: N) -> Result<Vec<std::net::IpAddr>, ResolveError> {
  let name = name.into_name()?;
  if unsafe { USE_IPV6 } {
    if let Ok(r) = AAAA(name.clone()).await {
      if !r.is_empty() {
        return Ok(r);
      }
    }
    UNSAFE {
      USE_IPV6 = false;
    }
  }
  A(name).await
}



