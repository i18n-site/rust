pub trait ConfTrait {
  fn system(&self) -> &str;
  fn temperature(&self) -> f32;
  fn reasoning_effort(&self) -> &Option<String> {
    &None
  }
  fn fmt(&self, txt: String) -> String {
    txt
  }
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
}

#[derive(Debug, Clone)]
pub struct ConfQroq {
  pub system: String,
  pub temperature: f32,
  pub reasoning_effort: Option<String>,
}

impl ConfQroq {
  pub fn new<S: Into<String>>(
    system: impl Into<String>,
    temperature: f32,
    reasoning_effort: Option<S>,
  ) -> ConfQroq {
    ConfQroq {
      system: system.into(),
      temperature,
      reasoning_effort: reasoning_effort.map(|i| i.into()),
    }
  }
}

impl ConfTrait for ConfQroq {
  fn system(&self) -> &str {
    &self.system
  }
  fn temperature(&self) -> f32 {
    self.temperature
  }
  fn reasoning_effort(&self) -> &Option<String> {
    &self.reasoning_effort
  }
}
