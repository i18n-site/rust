use hickory_resolver::{
  config::{ResolverConfig, ResolverOpts},
  TokioAsyncResolver,
};

#[static_init::dynamic]
pub static RESOLVER: [TokioAsyncResolver; 2] = [
  TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default()),
  TokioAsyncResolver::tokio(ResolverConfig::google(), ResolverOpts::default()),
];

//     match RESOLVER.lookup(host, RecordType::$rec_type).await {
// pub fn lookup() {}
