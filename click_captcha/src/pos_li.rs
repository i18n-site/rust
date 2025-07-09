use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::N;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pos {
  pub size: u32,
  pub x: u32,
  pub y: u32,
}

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct PosLi(pub [Pos; N]);
