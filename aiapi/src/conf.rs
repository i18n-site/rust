use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ReasoningEffort {
  #[default]
  Default,
  None,
  Minimal,
  Low,
  Medium,
  High,
}

pub trait ConfTrait {
  fn system(&self) -> &str;
  fn temperature(&self) -> f32;
  fn reasoning_effort(&self) -> ReasoningEffort {
    ReasoningEffort::None
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
  pub reasoning_effort: ReasoningEffort,
}

impl ConfQroq {
  pub fn new(
    system: impl Into<String>,
    temperature: f32,
    reasoning_effort: ReasoningEffort,
  ) -> ConfQroq {
    ConfQroq {
      system: system.into(),
      temperature,
      reasoning_effort,
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
  fn reasoning_effort(&self) -> ReasoningEffort {
    self.reasoning_effort.clone()
  }
}
