pub use const_str;
pub use static_init;
pub use tracing;

pub fn li_no_warn(name: impl AsRef<str>) -> Vec<String> {
  let mut r = Vec::new();
  let name = name.as_ref();
  if let Ok(val) = std::env::var(name) {
    for i in val.split_whitespace() {
      r.push(i.into());
    }
  }
  r
}
pub fn li(name: impl AsRef<str>) -> Vec<String> {
  let name = name.as_ref();
  let r = li_no_warn(name);
  if r.is_empty() {
    tracing::warn!("⚠️ ENV {name} IS EMPTY");
  }
  r
}

pub fn get<T: std::str::FromStr>(name: &str) -> T
where
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  let val = std::env::var(name);
  match val {
    Ok(val) => match val.parse() {
      Ok(r) => r,
      Err(err) => {
        panic!("❌ {name}={val} {:?}", err);
      }
    },
    Err(err) => {
      panic!("❌ {err} {name}");
    }
  }
}

pub fn get_or_default<T: std::str::FromStr>(name: &str, default: T) -> T
where
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  if let Ok(i) = std::env::var(name) {
    match i.parse() {
      Ok(i) => return i,
      Err(err) => tracing::error!("❌ ENV PARSE ERROR {name}={i} : {:?}", err),
    }
  }
  default
}

#[macro_export]
macro_rules! s {
($name:ident) => {
$crate::s!($name: String);
};
($name:ident: $ty:ty) => {
#[$crate::static_init::dynamic]
pub static $name: $ty = $crate::get(stringify!($name));
};
($($name:ident: $type:ty | $default:expr);+ $(;)?) => {
$(
  #[$crate::static_init::dynamic]
  pub static $name: $type = {

    let name = stringify!($name);
    std::env::var(name).map(
      |i|{
        match i.parse(){
          Ok(i)=>i,
          Err(err) => {
            panic!("❌ {name}={i} {:?}", err);
          }
        }
      }
    ).unwrap_or($default)
  };
)+
};
($($name:ident$(:$ty:ty)?),+) => {
$(
$crate::s!($name$(: $ty)?);
)+
};
}

#[macro_export]
macro_rules! def {
($($name:ident: $type:ty | $default:expr);+ $(;)?) => {
$(
#[allow(non_snake_case)]
pub fn $name() -> $type {
    $crate::get_or_default(stringify!($name), $default)
}
)+
};
($name:ident) => {
#[allow(non_snake_case)]
pub fn $name<T: std::str::FromStr>() -> T
    where <T as std::str::FromStr>::Err: std::fmt::Debug {
        $crate::get(stringify!($name))
}
};
($($name:ident),+) => {
$(
    $crate::def!($name);
)+
};
}
