// use tracing::info;
use std::collections::BTreeMap;

use aok::{OK, Void};
use postcard::{self, from_bytes};
use serde::{Deserialize, Serialize};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// 测试布尔类型
#[test]
fn test_bool() {
  let val = false;
  assert_eq!(
    from_bytes::<bool>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = true;
  assert_eq!(
    from_bytes::<bool>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 i8 类型
#[test]
fn test_i8() {
  let val = 127i8;
  assert_eq!(
    from_bytes::<i8>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = -128i8;
  assert_eq!(
    from_bytes::<i8>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 u8 类型
#[test]
fn test_u8() {
  let val = 255u8;
  assert_eq!(
    from_bytes::<u8>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 i16 类型
#[test]
fn test_i16() {
  let val = 63i16;
  assert_eq!(
    from_bytes::<i16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = -64i16;
  assert_eq!(
    from_bytes::<i16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = 64i16;
  assert_eq!(
    from_bytes::<i16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 u16 类型
#[test]
fn test_u16() {
  let val = 127u16;
  assert_eq!(
    from_bytes::<u16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = 128u16;
  assert_eq!(
    from_bytes::<u16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = 65535u16;
  assert_eq!(
    from_bytes::<u16>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 f32 类型
#[test]
fn test_f32() {
  let val = -32.005_86_f32;
  assert_eq!(
    from_bytes::<f32>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 char 类型
#[test]
fn test_char() {
  let val = 'A';
  assert_eq!(
    from_bytes::<char>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 &str 类型
#[test]
fn test_str() {
  let val = "Hell";
  assert_eq!(
    from_bytes::<&str>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试字节数组 &[u8] 类型
#[test]
fn test_bytes() {
  let val = [0u8, 0x01, 0x02, 0x03];
  let bin = postcard::to_allocvec(&val).unwrap();
  assert_eq!(from_bytes::<[u8; 4]>(&bin).unwrap(), val);

  let val = &val[..];
  let bin = postcard::to_allocvec(val).unwrap();
  assert_eq!(from_bytes::<&[u8]>(&bin).unwrap(), val);
}

// 测试 Option 类型
#[test]
fn test_option() {
  let val = None::<u8>;
  assert_eq!(
    from_bytes::<Option<u8>>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  let val = Some(255u8);
  assert_eq!(
    from_bytes::<Option<u8>>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// // 测试单元类型 ()
// #[test]
// fn test_unit() {
//   let val = ();
//   assert_eq!(
//     from_bytes::<()>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
//     val
//   );
// }

// 测试单元结构体
#[test]
fn test_unit_struct() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct Unit;
  let val = Unit;
  assert_eq!(
    from_bytes::<Unit>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 newtype 结构体
#[test]
fn test_newtype_struct() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct Newtype(u8);
  let val = Newtype(255);
  assert_eq!(
    from_bytes::<Newtype>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试序列 (Vec)
#[test]
fn test_seq() {
  let val = vec![1u8, 2, 3];
  assert_eq!(
    from_bytes::<Vec<u8>>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试元组
#[test]
fn test_tuple() {
  let val = (1u8, 2u8);
  assert_eq!(
    from_bytes::<(u8, u8)>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );
}

// 测试 Map
#[test]
fn test_map() {
  let mut val = BTreeMap::new();
  val.insert("a", 1);
  val.insert("b", 2);
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = from_bytes::<BTreeMap<&str, i32>>(&serialized).unwrap();
  assert_eq!(deserialized, val);
}

// 测试结构体
#[test]
fn test_struct() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct TestStruct {
    a: u8,
    b: u16,
    c: Option<i32>,
  }
  let val = TestStruct {
    a: 1,
    b: 2,
    c: Some(3),
  };
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = from_bytes::<TestStruct>(&serialized).unwrap();
  assert_eq!(deserialized, val);
}

// 测试枚举
#[test]
fn test_enum() -> Void {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  enum TestEnum {
    Unit,
    Newtype(u8),
    Tuple(u8, u16),
    Struct { c: i64, a: u8, b: u16 },
  }
  let val = TestEnum::Struct { a: 1, b: 2, c: -32 };
  let bin = &postcard::to_allocvec(&val)?;
  assert_eq!(from_bytes::<TestEnum>(bin)?, val);

  let val = TestEnum::Tuple(1, 2);
  let bin = &postcard::to_allocvec(&val)?;
  assert_eq!(from_bytes::<TestEnum>(bin)?, val);

  let val = TestEnum::Unit;
  let bin = &postcard::to_allocvec(&val)?;
  assert_eq!(from_bytes::<TestEnum>(bin)?, val);

  let val = TestEnum::Newtype(255);
  assert_eq!(
    from_bytes::<TestEnum>(&postcard::to_allocvec(&val).unwrap()).unwrap(),
    val
  );

  OK
}

// 测试不完整的元组的反序列化
#[test]
fn test_incomplete_tuple() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct IncompleteTuple(u8, u16);

  #[derive(Deserialize, PartialEq, Debug)]
  struct CompleteTuple(u8, u16, u32, Option<u64>);

  let val = IncompleteTuple(1, 2);
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = from_bytes::<CompleteTuple>(&serialized).unwrap();

  // 验证反序列化后的元组的各个字段
  assert_eq!(deserialized, CompleteTuple(1, 2, 0, None));
}

// 测试不完整的结构体的反序列化
#[test]
fn test_incomplete_struct() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct IncompleteStruct {
    a: i16,
    b: u8,
  }

  #[derive(Deserialize, PartialEq, Debug)]
  struct CompleteStruct {
    a: i16,
    b: u8,
    c: u32,
    d: Option<u64>,
  }

  let val = IncompleteStruct {
    a: -1231i16,
    b: 255,
  };
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = from_bytes::<CompleteStruct>(&serialized).unwrap();

  // 验证反序列化后的结构体的各个字段
  assert_eq!(
    deserialized,
    CompleteStruct {
      a: val.a,
      b: val.b,
      c: 0,
      d: None
    }
  );
}

// 测试不完整的序列 (Vec) 的反序列化
#[test]
fn test_incomplete_seq() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct IncompleteSeq(u8, u16);

  #[derive(Deserialize, PartialEq, Debug)]
  struct CompleteSeq(u8, u16, u32, Option<u64>);

  let val = vec![IncompleteSeq(1, 2), IncompleteSeq(3, 4)];
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = from_bytes::<Vec<CompleteSeq>>(&serialized).unwrap();
  assert_eq!(
    deserialized,
    vec![CompleteSeq(1, 2, 0, None), CompleteSeq(3, 4, 0, None)]
  );
}
