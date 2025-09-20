pub fn now_ms() -> u64 {
  coarsetime::Clock::now_since_epoch().as_millis()
}

pub struct CostTime {
  pub start: u64,
}

impl Default for CostTime {
  fn default() -> Self {
    Self { start: now_ms() }
  }
}

pub fn start() -> CostTime {
  CostTime::default()
}

impl CostTime {
  pub fn ms(&self) -> u64 {
    now_ms() - self.start
  }

  pub fn sec(&self) -> f32 {
    self.ms() as f32 / 1000.0
  }
}
