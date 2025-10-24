use crate::Response;

pub enum Result<T> {
  Ok(T),
  Err(anyhow::Error),
  Response(Response),
}

impl<T> From<T> for Result<T> {
  fn from(value: T) -> Self {
    Self::Ok(value)
  }
}

impl<T> From<anyhow::Result<T>> for Result<T> {
  fn from(value: anyhow::Result<T>) -> Self {
    match value {
      Ok(value) => Self::Ok(value),
      Err(err) => match err.downcast::<crate::Response>() {
        Ok(response) => Self::Response(response),
        Err(err) => Self::Err(err),
      },
    }
  }
}
