#[macro_export]
macro_rules! ip {
  ($ip1:expr, $ip2:expr, $ip3:expr, $ip4:expr) => {
    std::net::IpAddr::V4(std::net::Ipv4Addr::new($ip1, $ip2, $ip3, $ip4))
  };
  (
    $ip1:expr, $ip2:expr, $ip3:expr, $ip4:expr,
    $ip5:expr, $ip6:expr, $ip7:expr, $ip8:expr
  ) => {
    std::net::IpAddr::V6(std::net::Ipv6Addr::new(
      $ip1, $ip2, $ip3, $ip4, $ip5, $ip6, $ip7, $ip8,
    ))
  };
  (
    $(
      $($ip:expr),+
    );+
    $(;)?
  ) => {
    [
      $(
        ip!($($ip),+)
      ),+
    ]
  };
}
