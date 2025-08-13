use std::fmt::Display;

use sonic_rs::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Msg {
  pub role: String,
  pub content: String,
}

impl AsRef<Msg> for Msg {
  fn as_ref(&self) -> &Msg {
    self
  }
}

impl From<&(&str, String)> for Msg {
  fn from((role, content): &(&str, String)) -> Self {
    Self {
      role: (*role).into(),
      content: content.into(),
    }
  }
}

impl<S1: Into<String>, S2: Into<String>> From<(S1, S2)> for Msg {
  fn from((role, content): (S1, S2)) -> Self {
    Self {
      role: role.into(),
      content: content.into(),
    }
  }
}

#[derive(Debug)]
pub struct MsgLi(pub Vec<String>);

impl<I: IntoIterator<Item = M>, M: AsRef<Msg>> From<I> for MsgLi {
  fn from(li: I) -> Self {
    let li: Vec<String> = li
      .into_iter()
      .map(|i| sonic_rs::to_string(i.as_ref()).unwrap())
      .collect();
    Self(li)
  }
}

impl Display for MsgLi {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.join(","))
  }
}

impl From<MsgLi> for String {
  fn from(li: MsgLi) -> Self {
    li.0.join(",")
  }
}
