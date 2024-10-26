use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PayOff {
  pub cost: u64,
  pub asset: i64,
}
