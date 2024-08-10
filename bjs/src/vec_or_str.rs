use boa_engine::{
  object::builtins::{JsArrayBuffer, JsUint8Array},
  Context, JsValue,
};

#[derive(Debug)]
pub enum VecOrStr {
  Vec(Vec<u8>),
  Str(String),
}

impl VecOrStr {
  pub fn to_bytes(ctx: &mut Context, value: JsValue) -> Option<Vec<u8>> {
    Self::parse(ctx, value).map(|v| match v {
      VecOrStr::Vec(v) => v,
      VecOrStr::Str(s) => Vec::from(s.as_bytes()),
    })
  }

  pub fn parse(ctx: &mut Context, value: JsValue) -> Option<Self> {
    match value {
      JsValue::String(s) => Some(VecOrStr::Str(s.to_std_string_escaped())),
      JsValue::Object(o) => {
        if let Ok(uint8_array) = JsUint8Array::from_object(o)
          && let Ok(buf) = uint8_array.buffer(ctx)
          && let JsValue::Object(buf) = buf
          && let Ok(buf) = JsArrayBuffer::from_object(buf)
        {
          if let Some(buf) = buf.data() {
            return Some(VecOrStr::Vec(buf.as_ref().into()));
          }
        }
        None
      }
      _ => None,
    }
  }
}
