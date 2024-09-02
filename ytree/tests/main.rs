use std::{
  collections::HashMap,
  io::{BufRead, Cursor},
};

use aok::{Result, OK};
use lang::Lang;
use static_init::constructor;
use tracing::info;
use ytree::sitemap::lang_li_e;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let paths = vec![
    "README.md#a".to_string(),
    "blog/README.md#2".to_string(),
    "blog/news/README.md#c".to_string(),
    "blog/news/begin.md#d".to_string(),
    "x/news/1.md#x".to_string(),
    "x/2/3.md#y".to_string(),
  ];

  // let yml = serde_yaml::to_string(&root).unwrap();
  // info!("{}", yml);

  let mut bitmap = roaring::RoaringBitmap::new();
  for i in [Lang::Ja, Lang::En, Lang::Zh, Lang::ZhTw] {
    bitmap.insert(i as u32);
  }

  let yml = ytree::sitemap::dumps(HashMap::from_iter([(lang_li_e(&bitmap), paths)]));

  info!("{yml}");
  let cursor = Cursor::new(yml.as_bytes());

  let yml = ytree::sitemap::loads(cursor.lines().map_while(Result::ok));

  let t = yml.sitemap("/Users/z/i18n/md")?;
  for i in &t.rel_lang_set {
    info!("{} {:?}", i.0, i.1);
  }
  // for i in t.set() {
  //   println!("{i}");
  // }
  info!("{}", t.dumps());
  OK
}
