use bitcode::{Decode, Encode};
use derive_more::{Deref, DerefMut};

use crate::N;

#[derive(Debug, Encode, Decode)]
pub struct Pos {
  pub size: u32,
  pub x: u32,
  pub y: u32,
}

#[derive(Debug, Deref, DerefMut, Encode, Decode)]
pub struct PosLi(pub [Pos; N]);
