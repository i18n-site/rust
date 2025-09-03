# pc

[postcard](https://github.com/jamesmunns/postcard) deserializer , support incomple struct deserialize into complete struct

[postcard](https://github.com/jamesmunns/postcard) 序列化解码，支持把不完整类型的序列化解码到完整类型

```rust
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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
