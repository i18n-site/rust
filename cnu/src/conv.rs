use enum_dispatch::enum_dispatch;
use lang::Lang;

use crate::{f2j, j2f};

#[enum_dispatch]
pub trait Conv {
  fn conv(&self, s: impl AsRef<str>) -> String;
  fn li(&self, li: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<String> {
    li.into_iter().map(|i| self.conv(i)).collect()
  }
}

macro_rules! conv {
  ($($cls:ident: $fn:ident);+) => {

#[enum_dispatch(Conv)]
pub enum ConvEnum {
  $($cls),+
}

$(conv!($cls $fn);)+

  };

  ($cls:ident $fn:ident) => {
    pub struct $cls();
    impl Conv for $cls {
      fn conv(&self, s: impl AsRef<str>) -> String {
        $fn(s)
      }
    }
  };
}

conv!(
  Fj: f2j;
  Jf: j2f
);

pub fn conv(from_lang: Lang, to_lang: Lang) -> Option<ConvEnum> {
  Some(if from_lang == Lang::ZhTw && to_lang == Lang::Zh {
    ConvEnum::Fj(Fj())
  } else if from_lang == Lang::Zh && to_lang == Lang::ZhTw {
    ConvEnum::Jf(Jf())
  } else {
    return None;
  })
}
