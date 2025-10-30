use bincode::{Decode, Encode, config, config::Configuration, decode_from_slice, encode_to_vec};

pub static CONF: Configuration = config::standard();

pub fn e<T: Encode>(t: T) -> Result<Vec<u8>, bincode::error::EncodeError> {
  encode_to_vec(t, CONF)
}

pub fn d<T: Decode<()>>(t: impl AsRef<[u8]>) -> Result<T, bincode::error::DecodeError> {
  decode_from_slice(t.as_ref(), CONF).map(|i| i.0)
}
