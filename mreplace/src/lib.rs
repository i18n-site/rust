#![feature(slice_index_methods)]

pub use daachorse::{CharwiseDoubleArrayAhoCorasick, errors::Result};

pub struct Mreplace {
  pub ac: CharwiseDoubleArrayAhoCorasick<usize>,
}

impl Mreplace {
  pub fn new<S: ToString>(from_string: impl IntoIterator<Item = S>) -> Result<Self> {
    let li: Vec<(String, usize)> = from_string
      .into_iter()
      .enumerate()
      .map(|(pos, i)| (i.to_string(), pos))
      .collect();

    Ok(Self {
      ac: CharwiseDoubleArrayAhoCorasick::with_values(li)?,
    })
  }
  pub fn replace<S: AsRef<str>>(&self, txt: impl AsRef<str>, to_string: impl AsRef<[S]>) -> String {
    let to_string = to_string.as_ref();
    let txt = txt.as_ref();
    let mut r = String::new();
    let mut pre = 0;
    for i in self.ac.find_iter(txt) {
      r.push_str(&txt[pre..i.start()]);
      r.push_str(to_string[i.value()].as_ref());
      pre = i.end();
    }
    r.push_str(&txt[pre..txt.len()]);
    r
  }
}

#[cfg(feature = "macro")]
pub use const_str;

#[cfg(feature = "macro")]
#[macro_export]
macro_rules! mreplace {
  ($($var:ident : $($k:ident)+);*) => {
  $(
  #[static_init::dynamic]
  static $var: $crate::Mreplace = $crate::Mreplace::new([
    $($crate::const_str::concat!("${",stringify!($k),"}")),+
  ]).unwrap();
  )*
  };
  ($($var:ident : $($k:ident)+;)*) => {
    $crate::mreplace!($($var:$($k)+);*);
  };
}
