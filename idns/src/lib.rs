use core::net::Ipv4Addr;
use std::{net::IpAddr, time::Duration};

use hickory_proto::rr::record_type::RecordType;
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

#[static_init::dynamic]
pub static RESOLVER: [TokioAsyncResolver; 2] = {
  let mut opt = ResolverOpts::default();
  opt.attempts = 6;
  opt.num_concurrent_reqs = 3;
  opt.timeout = Duration::from_secs(6);
  [
    TokioAsyncResolver::tokio(
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
          false,
        ),
      ),
      opt.clone(),
    ),
    TokioAsyncResolver::tokio(ResolverConfig::cloudflare_tls(), opt),
  ]
};

pub async fn lookup<N: IntoName>(name: N, record_type: RecordType) -> Result<Lookup, ResolveError> {
  let name = name.into_name()?;
  match RESOLVER[0].lookup(name.clone(), record_type).await {
    Ok(r) => Ok(r),
    Err(err) => {
      tracing::warn!("DNS ERROR: {}", err);
      RESOLVER[1].lookup(name, record_type).await
    }
  }
}
//     match RESOLVER.lookup(host, RecordType::$rec_type).await {
// #[static_init::dynamic]
// pub static RESOLVER: NameServerConfigGroup =
//   NameServerConfigGroup::from_ips_clear(&[IpAddr::V4(Ipv4Addr::new(223, 5, 5, 5))], 53, true);

// from_ips_clear
