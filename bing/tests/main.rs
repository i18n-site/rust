use std::time::Duration;

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
    txt: String::from_utf8_lossy(include_bytes!("./blog.md")).into(),
  };

  let doc_id = writer.add(doc1)?;
  dbg!(doc_id);

  let mut seacher = db.searcher()?;
  let li = seacher.search(bing::search::Query {
    query: "品牌".into(),
    uid: 0,
    org_id: 0,
    repo_id_li: vec![],
    tag_li: vec![],
    ts_begin: None,
    ts_end: None,
    limit: 10,
    offset: 0,
    snippet_max_num_chars: 300,
  })?;
  dbg!(li);

  writer.rm(id)?;

  std::thread::sleep(Duration::from_secs(1));
  let li = seacher.search(bing::search::Query {
    query: "品牌".into(),
    ..Default::default()
  })?;
  dbg!(li);

  Ok(())
}
