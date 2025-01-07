use std::mem::transmute;

pub mod cn;
pub use cn::LANG_CN;
pub use strum::IntoEnumIterator;
mod case;
pub use case::CASE;
mod nospace;
pub use nospace::NOSPACE;
mod code_id;
pub use code_id::CODE_ID;
mod lang_name;
pub use lang_name::LANG_NAME;
mod lang;
pub use lang::Lang;
pub use lang_code::LANG_CODE;
mod lang_code;

pub const EN_NO_TRAN: &[Lang] = &[Lang::Zh, Lang::ZhTw, Lang::Ja, Lang::Ko];

pub fn code_lang(code: impl AsRef<str>) -> Option<Lang> {
  LANG_CODE
    .iter()
    .position(|&x| x == code.as_ref())
    .map(|x| unsafe { transmute(x as u16) })
}

pub fn lang_code(lang: lang::Lang) -> &'static str {
  LANG_CODE[lang as usize]
}

impl Lang {
  pub fn code(&self) -> &'static str {
    LANG_CODE[*self as usize]
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("no match")]
  NoMatch,
}

impl TryFrom<&str> for Lang {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match code_lang(value) {
      Some(r) => Ok(r),
      None => Err(Error::NoMatch),
    }
  }
}

impl TryFrom<&String> for Lang {
  type Error = Error;

  fn try_from(val: &String) -> Result<Self, Self::Error> {
    val.as_str().try_into()
  }
}

impl TryFrom<String> for Lang {
  type Error = Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let val: &str = value.as_ref();
    val.try_into()
  }
}

impl From<&Lang> for Lang {
  fn from(lang: &Lang) -> Self {
    *lang
  }
}
