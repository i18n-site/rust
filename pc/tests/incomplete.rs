use serde::{Deserialize, Serialize};

// 测试不完整的元组的反序列化
#[test]
fn test_incomplete_tuple() {
  #[derive(Deserialize, Serialize, Debug, PartialEq)]
  struct IncompleteTuple(u8, u16);

  #[derive(Deserialize, PartialEq, Debug)]
  struct CompleteTuple(u8, u16, u32, Option<u64>);

  let val = IncompleteTuple(1, 2);
  let serialized = postcard::to_allocvec(&val).unwrap();
  let deserialized = pc::d::<CompleteTuple>(&serialized).unwrap();

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
  let deserialized = pc::d::<CompleteStruct>(&serialized).unwrap();

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
  let deserialized = pc::d::<Vec<CompleteSeq>>(&serialized).unwrap();
  assert_eq!(
    deserialized,
    vec![CompleteSeq(1, 2, 0, None), CompleteSeq(3, 4, 0, None)]
  );
}
