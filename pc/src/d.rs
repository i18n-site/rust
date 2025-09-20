use paste::paste;
use serde::{
  de::{
    self, DeserializeSeed, Deserializer, EnumAccess, Error, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
  },
  Deserialize,
};

macro_rules! read_varint {
    ($self:ident, $type:ty, $max_shift:expr) => {
        paste! {
          fn [<read_varint_ $type>](&mut $self) -> Result<$type, de::value::Error> {
            let mut result: $type = 0;
            let mut shift = 0;
            loop {
              let Some(byte) = $self.read_byte() else {
                break
              };
              result |= ((byte & 0x7F) as $type) << shift;
              if (byte & 0x80) == 0 {
                return Ok(result);
              }
              shift += 7;
              if shift > $max_shift {
                return Err(de::value::Error::custom(concat!("varint too large for ", stringify!($type))));
              }
            }
            Err(
              de::value::Error::custom(concat!("varint too short for ", stringify!($type)))
            )
          }
        }
    };
}

struct PostcardDecoder<'de> {
  input: &'de [u8],
  pos: usize,
}

impl<'de> PostcardDecoder<'de> {
  fn new(input: &'de [u8]) -> Self {
    PostcardDecoder { input, pos: 0 }
  }

  // 读取一个字节
  fn read_byte(&mut self) -> Option<u8> {
    if self.pos < self.input.len() {
      let b = self.input[self.pos];
      self.pos += 1;
      Some(b)
    } else {
      None
    }
  }

  // 查看下一个字节，但不消耗
  fn peek_byte(&self) -> Option<u8> {
    if self.pos < self.input.len() {
      Some(self.input[self.pos])
    } else {
      None
    }
  }

  // 读取指定长度的字节切片
  fn read_bytes(&mut self, len: usize) -> &'de [u8] {
    let end = core::cmp::min(self.pos + len, self.input.len());
    let bytes = &self.input[self.pos..end];
    self.pos = end;
    bytes
  }

  // 读取 i8
  fn read_i8(&mut self) -> Option<i8> {
    self.read_byte().map(|b| b as i8)
  }

  // 读取 u8
  fn read_u8(&mut self) -> Option<u8> {
    self.read_byte()
  }

  // // 填充默认值，并移动指针
  // fn fill_default<T: Default>(&mut self, len: usize) -> T {
  //   self.pos = core::cmp::min(self.pos + len, self.input.len());
  //   Default::default()
  // }

  read_varint!(self, u16, 14);
  read_varint!(self, u32, 28);
  read_varint!(self, u64, 56);
  read_varint!(self, u128, 126);

  // 读取 usize
  fn read_usize(&mut self) -> Result<usize, de::value::Error> {
    #[cfg(target_pointer_width = "64")]
    {
      self.read_varint_u64().map(|v| v as usize)
    }
    #[cfg(target_pointer_width = "32")]
    {
      self.read_varint_u32().map(|v| v as usize)
    }
    #[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
    {
      compile_error!("Unsupported target_pointer_width")
    }
  }

  // // 读取 isize
  // fn read_isize(&mut self) -> Result<isize, de::value::Error> {
  //   #[cfg(target_pointer_width = "64")]
  //   {
  //     self.read_varint_u64().map(|v| de_zig_zag_i64(v) as isize)
  //   }
  //   #[cfg(target_pointer_width = "32")]
  //   {
  //     self.read_varint_u32().map(|v| de_zig_zag_i32(v) as isize)
  //   }
  //   #[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
  //   {
  //     compile_error!("Unsupported target_pointer_width")
  //   }
  // }
}

// ZigZag 解码 i16
fn de_zig_zag_i16(n: u16) -> i16 {
  ((n >> 1) as i16) ^ (-((n & 0b1) as i16))
}

// ZigZag 解码 i32
fn de_zig_zag_i32(n: u32) -> i32 {
  ((n >> 1) as i32) ^ (-((n & 0b1) as i32))
}

// ZigZag 解码 i64
fn de_zig_zag_i64(n: u64) -> i64 {
  ((n >> 1) as i64) ^ (-((n & 0b1) as i64))
}

// ZigZag 解码 i128
fn de_zig_zag_i128(n: u128) -> i128 {
  ((n >> 1) as i128) ^ (-((n & 0b1) as i128))
}

macro_rules! read_varint_signed {
  ( $type:ty, $unsigned_type:ty, $max_shift:expr) => {
    paste! {
      fn [<read_varint_ $type>](&mut self) -> Result<$type, de::value::Error> {
        let raw: $unsigned_type = self.[<read_varint_ $unsigned_type>]()?;
        Ok(paste! { [<de_zig_zag_ $type>]}(raw))
      }
    }
  };
}

impl PostcardDecoder<'_> {
  read_varint_signed!(i16, u16, 14);
  read_varint_signed!(i32, u32, 28);
  read_varint_signed!(i64, u64, 56);
  read_varint_signed!(i128, u128, 126);
}

impl<'de> Deserializer<'de> for &mut PostcardDecoder<'de> {
  type Error = de::value::Error;

  fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    // Postcard 不支持反序列化 "any" 类型
    Err(de::value::Error::custom(
      "Postcard does not support deserialize_any",
    ))
  }

  fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    match self.read_byte() {
      Some(0) => visitor.visit_bool(false),
      Some(1) => visitor.visit_bool(true),
      _ => visitor.visit_bool(false), // 输入无效或结束，默认为 false
    }
  }

  fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i8(self.read_i8().unwrap_or_default())
  }

  fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i16(self.read_varint_i16().unwrap_or_default())
  }

  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i32(self.read_varint_i32().unwrap_or_default())
  }

  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i64(self.read_varint_i64().unwrap_or_default())
  }

  fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i128(self.read_varint_i128().unwrap_or_default())
  }

  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u8(self.read_u8().unwrap_or_default())
  }

  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u16(self.read_varint_u16().unwrap_or_default())
  }

  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u32(self.read_varint_u32().unwrap_or_default())
  }

  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u64(self.read_varint_u64().unwrap_or_default())
  }

  fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u128(self.read_varint_u128().unwrap_or_default())
  }

  fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let bytes = self.read_bytes(4);
    if bytes.len() < 4 {
      visitor.visit_f32(f32::default())
    } else {
      let bits = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
      visitor.visit_f32(f32::from_bits(bits))
    }
  }

  fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let bytes = self.read_bytes(8);
    if bytes.len() < 8 {
      visitor.visit_f64(f64::default())
    } else {
      let bits = u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
      ]);
      visitor.visit_f64(f64::from_bits(bits))
    }
  }

  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    // 这里改变了策略，先读长度，再读内容。更符合 postcard 的要求
    let len = self.read_usize().unwrap_or_default();
    let bytes = self.read_bytes(len);
    let s = core::str::from_utf8(bytes).unwrap_or_default();
    // 只取第一个字符, 其余的字符被丢弃, 这其实不满足 postcard 的规范，但是目前先这么做
    let mut chars = s.chars();
    visitor.visit_char(chars.next().unwrap_or_default())
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let len = self.read_usize().unwrap_or_default();
    let bytes = self.read_bytes(len);
    // 这里用了 visit_borrowed_str, 因为我们直接用了输入数据的切片
    match core::str::from_utf8(bytes) {
      Ok(s) => visitor.visit_borrowed_str(s),
      Err(_) => visitor.visit_borrowed_str(""), // 错误的 UTF-8 字节，默认为空字符串
    }
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    // 对于 String 类型，我们需要拥有所有权，因此需要复制数据
    self.deserialize_str(visitor)
  }

  fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let len = self.read_usize().unwrap_or_default();
    let bytes = self.read_bytes(len);
    // 这里用了 visit_borrowed_bytes, 因为我们直接用了输入数据的切片
    visitor.visit_borrowed_bytes(bytes)
  }

  fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    // 对于 ByteBuf，我们需要拥有所有权，因此需要复制数据
    self.deserialize_bytes(visitor)
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    match self.read_byte() {
      Some(0) => visitor.visit_none(),
      Some(1) => visitor.visit_some(self),
      _ => visitor.visit_none(), // 输入无效或结束，默认为 None
    }
  }

  fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.deserialize_unit(visitor)
  }

  fn deserialize_newtype_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let len = self.read_usize().unwrap_or_default();
    visitor.visit_seq(SeqReader::new(self, len))
  }

  fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(TupleReader::new(self))
  }

  fn deserialize_tuple_struct<V>(
    self,
    _name: &'static str,
    _len: usize, // 这个参数不再使用
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(TupleReader::new(self)) // 直接使用 TupleReader
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let len = self.read_usize().unwrap_or_default();
    visitor.visit_map(MapReader::new(self, len))
  }

  fn deserialize_struct<V>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(StructReader::new(self))
  }

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_enum(Enum::new(self))
  }

  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.deserialize_str(visitor)
  }

  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.deserialize_any(visitor)
  }

  fn is_human_readable(&self) -> bool {
    false
  }
}

struct SeqReader<'a, 'de: 'a> {
  de: &'a mut PostcardDecoder<'de>,
  len: usize,
  consumed: usize,
}

impl<'a, 'de> SeqReader<'a, 'de> {
  fn new(de: &'a mut PostcardDecoder<'de>, len: usize) -> Self {
    SeqReader {
      de,
      len,
      consumed: 0,
    }
  }
}

impl<'de> SeqAccess<'de> for SeqReader<'_, 'de> {
  type Error = de::value::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    if self.consumed < self.len {
      self.consumed += 1;
      seed.deserialize(&mut *self.de).map(Some)
    } else {
      Ok(None)
    }
  }
}

struct TupleReader<'a, 'de: 'a> {
  de: &'a mut PostcardDecoder<'de>,
}

impl<'a, 'de> TupleReader<'a, 'de> {
  fn new(de: &'a mut PostcardDecoder<'de>) -> Self {
    TupleReader { de }
  }
}

impl<'de> SeqAccess<'de> for TupleReader<'_, 'de> {
  type Error = de::value::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    if self.de.peek_byte().is_some() {
      // 如果还有数据，尝试反序列化
      seed.deserialize(&mut *self.de).map(Some)
    } else {
      // 如果数据不足，使用默认值
      seed.deserialize(DefaultValueDeserializer).map(Some)
    }
  }
}

struct StructReader<'a, 'de: 'a> {
  de: &'a mut PostcardDecoder<'de>,
}

impl<'a, 'de> StructReader<'a, 'de> {
  fn new(de: &'a mut PostcardDecoder<'de>) -> Self {
    StructReader { de }
  }
}

impl<'de> SeqAccess<'de> for StructReader<'_, 'de> {
  type Error = de::value::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    if self.de.peek_byte().is_some() {
      seed.deserialize(&mut *self.de).map(Some)
    } else {
      // 剩下的结构体字段填充默认值
      seed.deserialize(DefaultValueDeserializer).map(Some)
    }
  }
}

struct MapReader<'a, 'de: 'a> {
  de: &'a mut PostcardDecoder<'de>,
  len: usize,
  consumed: usize,
}

impl<'a, 'de> MapReader<'a, 'de> {
  fn new(de: &'a mut PostcardDecoder<'de>, len: usize) -> Self {
    MapReader {
      de,
      len,
      consumed: 0,
    }
  }
}

impl<'de> MapAccess<'de> for MapReader<'_, 'de> {
  type Error = de::value::Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where
    K: de::DeserializeSeed<'de>,
  {
    if self.consumed < self.len {
      self.consumed += 1;
      seed.deserialize(&mut *self.de).map(Some)
    } else {
      Ok(None)
    }
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    seed.deserialize(&mut *self.de)
  }
}

struct Enum<'a, 'de: 'a> {
  de: &'a mut PostcardDecoder<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
  fn new(de: &'a mut PostcardDecoder<'de>) -> Self {
    Enum { de }
  }
}

impl<'de> EnumAccess<'de> for Enum<'_, 'de> {
  type Error = de::value::Error;
  type Variant = Self;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    // 读取枚举的 discriminant (u32 类型)
    let discriminant = self.de.read_varint_u32().unwrap_or_default();
    // 使用 discriminant 作为反序列化器来反序列化枚举的 variant
    let val = seed.deserialize(discriminant.into_deserializer())?;
    Ok((val, self))
  }
}

impl<'de> VariantAccess<'de> for Enum<'_, 'de> {
  type Error = de::value::Error;

  fn unit_variant(self) -> Result<(), Self::Error> {
    Ok(())
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    seed.deserialize(self.de)
  }

  // 读取 tuple 变体
  fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(TupleReader::new(self.de))
  }

  fn struct_variant<V>(
    self,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    // 改为调用 deserialize_struct
    de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
  }
}

// 用于填充默认值的反序列化器
struct DefaultValueDeserializer;

impl<'de> Deserializer<'de> for DefaultValueDeserializer {
  type Error = de::value::Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_bool(bool::default())
  }

  fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i8(i8::default())
  }

  fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i16(i16::default())
  }

  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i32(i32::default())
  }

  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i64(i64::default())
  }

  fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_i128(i128::default())
  }

  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u8(u8::default())
  }

  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u16(u16::default())
  }

  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u32(u32::default())
  }

  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u64(u64::default())
  }

  fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_u128(u128::default())
  }

  fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_f32(f32::default())
  }

  fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_f64(f64::default())
  }

  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_char(char::default())
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_borrowed_str("")
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_string(String::default())
  }

  fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_borrowed_bytes(&[])
  }

  fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_byte_buf(Vec::default())
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_none()
  }

  fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn deserialize_newtype_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(DefaultSeqAccess)
  }

  fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(DefaultSeqAccess)
  }

  fn deserialize_tuple_struct<V>(
    self,
    _name: &'static str,
    _len: usize,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(DefaultSeqAccess)
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_map(DefaultMapAccess)
  }

  fn deserialize_struct<V>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(DefaultSeqAccess)
  }

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_enum(DefaultEnumAccess)
  }

  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_str("")
  }

  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn is_human_readable(&self) -> bool {
    false
  }
}

// 用于填充默认值的 SeqAccess
struct DefaultSeqAccess;

impl<'de> SeqAccess<'de> for DefaultSeqAccess {
  type Error = de::value::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where
    T: DeserializeSeed<'de>,
  {
    Ok(Some(seed.deserialize(DefaultValueDeserializer)?))
  }
}

// 用于填充默认值的 MapAccess
struct DefaultMapAccess;

impl<'de> MapAccess<'de> for DefaultMapAccess {
  type Error = de::value::Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where
    K: DeserializeSeed<'de>,
  {
    Ok(Some(seed.deserialize(DefaultValueDeserializer)?))
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where
    V: DeserializeSeed<'de>,
  {
    seed.deserialize(DefaultValueDeserializer)
  }
}

// 用于填充默认值的 EnumAccess
struct DefaultEnumAccess;

impl<'de> EnumAccess<'de> for DefaultEnumAccess {
  type Error = de::value::Error;
  type Variant = DefaultVariantAccess;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
  where
    V: DeserializeSeed<'de>,
  {
    Ok((
      seed.deserialize(0u32.into_deserializer())?, // 使用 0 作为默认的 discriminant
      DefaultVariantAccess,
    ))
  }
}

// 用于填充默认值的 VariantAccess
struct DefaultVariantAccess;

impl<'de> VariantAccess<'de> for DefaultVariantAccess {
  type Error = de::value::Error;

  fn unit_variant(self) -> Result<(), Self::Error> {
    Ok(())
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
  where
    T: DeserializeSeed<'de>,
  {
    seed.deserialize(DefaultValueDeserializer)
  }

  fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_seq(DefaultSeqAccess)
  }

  fn struct_variant<V>(
    self,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_map(DefaultMapAccess)
  }
}

pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, de::value::Error>
where
  T: Deserialize<'de>,
{
  let mut decoder = PostcardDecoder::new(input);
  T::deserialize(&mut decoder)
}
