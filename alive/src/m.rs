#[allow(non_snake_case,clippy::too_many_arguments)]
mod r#fn {
  pub use mysql_macro::*;

pub async fn argId(val:impl AsRef<str>)->Result<u64>{
  Ok(q1!("SELECT argId(?)",val.as_ref()))
}

#[macro_export]
macro_rules! argId {
($val:expr) => {
$crate::argId($val).await?
};
}

pub async fn hostId(val:impl AsRef<str>)->Result<u64>{
  Ok(q1!("SELECT hostId(?)",val.as_ref()))
}

#[macro_export]
macro_rules! hostId {
($val:expr) => {
$crate::hostId($val).await?
};
}

}

pub use r#fn::*;