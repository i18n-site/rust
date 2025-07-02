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
