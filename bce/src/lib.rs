use bincode::config::Configuration;
pub use bincode::*;

pub static CONF: Configuration = config::standard();

pub fn e<T: Encode>(t: T) -> Result<Vec<u8>, bincode::error::EncodeError> {
  encode_to_vec(t, CONF)
}

pub fn d<T: Decode>(t: &[u8]) -> Result<T, bincode::error::DecodeError> {
  decode_from_slice(t, CONF).map(|i| i.0)
}
