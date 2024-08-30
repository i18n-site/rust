use std::io::{BufRead, Cursor};

use aok::{Result, OK};
use lang::Lang;
use static_init::constructor;
use tracing::info;
use ytree::{sitemap::lang_li_e, Li};

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let paths = [
    "README.md#a",
    "blog/README.md#2",
    "blog/news/README.md#c",
    "blog/news/begin.md#d",
    "x/news/1.md#x",
    "x/2/3.md#y",
  ];

  let mut root = Li(Vec::new());

  for path in paths {
    root.push(path);
  }

  // root.remove(paths[1]);
  // root.remove(paths[0]);

  for i in root.iter() {
    info!("{i}");
  }

  // let yml = serde_yaml::to_string(&root).unwrap();
  // info!("{}", yml);

  let mut bitmap = roaring::RoaringBitmap::new();
  for i in [Lang::Ja, Lang::En, Lang::Zh, Lang::ZhTw] {
    bitmap.insert(i as u32);
  }

  let yml = ytree::sitemap::dumps([(lang_li_e(&bitmap), root)]);

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
