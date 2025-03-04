#[test]
fn test() {
  use xtld::host_port_tld;
  for d in [
    "a.中国.cn",
    "重.我.公司:3222",
    "a.b.c.com:322",
    "a.b.c.me",
    "a.b.com.cn",
  ] {
    println!("{} → {}", d, host_port_tld(d))
  }
}
