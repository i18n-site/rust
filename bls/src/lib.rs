use std::fmt;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use blake3::Hasher;
use bls12_381::{G1Affine, G1Projective, Scalar};
use group::{ff::Field, Curve};
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct SecretKey(pub [u8; 32]);

#[derive(Debug)]
pub struct PublicKey(pub [u8; 48]);

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("invalid public key")]
  PublicKey,
  #[error("invalid secret key")]
  SecretKey,
}

impl From<&SecretKey> for Scalar {
  fn from(sk: &SecretKey) -> Scalar {
    Scalar::from_bytes(&sk.0).unwrap()
  }
}

impl TryFrom<&PublicKey> for G1Projective {
  type Error = Error;
  fn try_from(pk: &PublicKey) -> Result<G1Projective, Error> {
    let pk = G1Affine::from_compressed(&pk.0);
    if pk.is_some().into() {
      return Ok(pk.unwrap().into());
    }
    Err(Error::PublicKey)
  }
}

impl Default for SecretKey {
  fn default() -> Self {
    let sk_scalar = Scalar::random(&mut OsRng);
    let sk_bytes = sk_scalar.to_bytes();
    SecretKey(sk_bytes)
  }
}

impl SecretKey {
  pub fn pk(&self) -> PublicKey {
    let sk_scalar = Scalar::from_bytes(&self.0).unwrap();
    let pk_point = G1Projective::generator() * sk_scalar;
    let pk_bytes = pk_point.to_affine().to_compressed();
    PublicKey(pk_bytes)
  }
}

pub fn sign(sk: impl Into<Scalar>, data: impl AsRef<[u8]>) -> [u8; 32] {
  let sk = sk.into();
  let data_scalar = hash(data);
  let signature_scalar = sk * data_scalar;
  signature_scalar.to_bytes()
}

pub fn verify(pk: impl Into<G1Projective>, data: impl AsRef<[u8]>, signature: &[u8; 32]) -> bool {
  let signature_scalar = Scalar::from_bytes(signature);
  if signature_scalar.is_some().into() {
    let pk = pk.into();
    let data_scalar = hash(data);
    let expected_point = G1Projective::generator() * signature_scalar.unwrap();
    let actual_point = pk * data_scalar;
    return expected_point == actual_point;
  }
  false
}

pub fn hash(data: impl AsRef<[u8]>) -> Scalar {
  let mut hasher = Hasher::new();
  hasher.update(data.as_ref());
  let mut output = [0; 64];
  hasher.finalize_xof().fill(&mut output);
  Scalar::from_bytes_wide(&output)
}

impl fmt::Display for SecretKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", URL_SAFE_NO_PAD.encode(self.0))
  }
}

impl fmt::Display for PublicKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", URL_SAFE_NO_PAD.encode(self.0))
  }
}

impl AsRef<[u8]> for SecretKey {
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl AsRef<[u8]> for PublicKey {
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}
