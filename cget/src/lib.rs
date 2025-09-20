#[macro_export]
macro_rules! cget {
  ($m:ident : $($var:ident: $ty:ty);* $(;)?) => {
    $(
      let $var = $m.get_one::<$ty>(stringify!($var)).unwrap();
    )*
  };
}
