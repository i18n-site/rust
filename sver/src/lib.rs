use std::fmt::{Debug, Display};

#[derive(Default, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Ver(pub [u32; 3]);

impl Display for Ver {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let li = self.0;
    write!(f, "{}.{}.{}", li[0], li[1], li[2])
  }
}

impl Debug for Ver {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let li = self.0;
    write!(f, "Ver({}.{}.{})", li[0], li[1], li[2])
  }
}

impl<S: AsRef<str>> From<S> for Ver {
  fn from(s: S) -> Self {
    let mut s = s.as_ref();
    macro_rules! parse {
      () => {{
        if let Some(p) = s.find('.') {
          let r = s[..p].parse().unwrap_or(0);
          s = &s[p + 1..];
          r
        } else {
          0
        }
      }};
    }
    let major = parse!();
    let minor = parse!();
    let patch = s.parse().unwrap_or(0);
    Ver([major, minor, patch])
  }
}
