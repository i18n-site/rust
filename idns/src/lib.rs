use core::net::Ipv4Addr;
use std::net::IpAddr;

use hickory_resolver::{
  config::{NameServerConfigGroup, ResolverConfig, ResolverOpts},
  TokioAsyncResolver,
};

// #[static_init::dynamic]
// pub static RESOLVER: [TokioAsyncResolver; 4] = [
// TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default()),
// TokioAsyncResolver::tokio(ResolverConfig::google_tls(), ResolverOpts::default()),
// TokioAsyncResolver::tokio(ResolverConfig::google(), ResolverOpts::default()),
// TokioAsyncResolver::tokio(ResolverConfig::cloudflare_tls(), ResolverOpts::default()),
// ];

//     match RESOLVER.lookup(host, RecordType::$rec_type).await {
// pub fn lookup() {}
#[static_init::dynamic]
pub static RESOLVER: NameServerConfigGroup =
  NameServerConfigGroup::from_ips_clear(&[IpAddr::V4(Ipv4Addr::new(223, 5, 5, 5))], 53, true);

// from_ips_clear
