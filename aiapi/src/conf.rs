pub trait ConfTrait {
  fn system(&self) -> &str;
  fn temperature(&self) -> f32;
  fn fmt(&self, txt: String) -> String;
}

#[derive(Debug, Clone)]
pub struct Conf {
  pub system: String,
  pub temperature: f32,
}

impl ConfTrait for Conf {
  fn system(&self) -> &str {
    &self.system
  }
  fn temperature(&self) -> f32 {
    self.temperature
  }
  fn fmt(&self, txt: String) -> String {
    txt
  }
}

#[derive(Debug, Clone)]
pub struct ConfNoThink {
  pub system: String,
  pub temperature: f32,
}

impl ConfNoThink {
  pub fn new(system: impl AsRef<str>, temperature: f32) -> ConfNoThink {
    let system = system.as_ref();
    ConfNoThink {
      system: if system.is_empty() {
        "/no-think".into()
      } else {
        format!("{system}  /no-think")
      },
      temperature,
    }
  }
}

impl ConfTrait for ConfNoThink {
  fn system(&self) -> &str {
    &self.system
  }
  fn temperature(&self) -> f32 {
    self.temperature
  }
  fn fmt(&self, txt: String) -> String {
    if let Some(p) = txt.find("</think>") {
      return txt[p + 8..].trim_start().into();
    }
    txt
  }
}
