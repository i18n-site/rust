mod lang;
pub use lang::Lang;
mod code_lang;
pub use lang_code::LANG_CODE;
mod lang_code;

pub use code_lang::CODE_LANG;

pub fn code_lang(code: impl AsRef<str>) -> Option<Lang> {
  let code = code.as_ref().to_lowercase();
  CODE_LANG.get(&code).cloned()
}

pub fn code_lang_unwrap(code: impl AsRef<str>) -> Lang {
  *CODE_LANG.get(code.as_ref()).unwrap()
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

impl TryFrom<String> for Lang {
  type Error = Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let val: &str = value.as_ref();
    val.try_into()
  }
}
