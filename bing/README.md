# bing : a search engine

```rust
use anyhow::Result;
use bing::Doc;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_search_engine() -> Result<()> {
  let path = "/tmp/bing";

  let db = bing::doc::open(path)?;

  let mut writer = db.writer()?;

  let id = 99;

  let doc1 = Doc {
    id,
    uid: 100,
    org_id: 2,
    repo_id: 10,
    tag_li: vec!["abc".into(), "电动车品牌".into(), "xyz".into()],
    ts: 1640995200, // 2022-01-01
    title: "YES Good 搜索引擎".into(),
    txt: "这是一个基于Tantivy的Rust搜索引擎，支持中文分词".into(),
  };

  let doc_id = writer.add(doc1)?;
  dbg!(doc_id);

  let mut seacher = db.searcher()?;
  let li = seacher.search("品牌", 0, 0, vec![], [], None, None, 10, 0)?;
  dbg!(li);

  writer.rm(id)?;

  let mut seacher = db.searcher()?;
  let li = seacher.search("品牌", 0, 0, vec![], [], None, None, 10, 0)?;
  dbg!(li);

  Ok(())
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
