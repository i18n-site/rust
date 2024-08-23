use std::io::{BufRead, Cursor};

use aok::{Result, OK};
use lang::Lang;
use static_init::constructor;
use tracing::info;
use ytree::{lang::lang_li_e, Li};

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let paths = [
    "README.md",
    "blog/README.md",
    "blog/news/README.md",
    "blog/news/begin.md",
    "x/news/1.md",
    "x/2/3.md",
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

  let yml = serde_yaml::to_string(&root).unwrap();
  info!("{}", yml);

  let mut bitmap = roaring::RoaringBitmap::new();
  for i in [Lang::Ja, Lang::En, Lang::Zh, Lang::ZhTw] {
    bitmap.insert(i as u32);
  }

  let yml = ytree::lang::dumps([(lang_li_e(&bitmap), root)]);

  info!("{yml}");
  let cursor = Cursor::new(yml.as_bytes());

  let yml = ytree::lang::loads(cursor.lines().filter_map(|i| i.ok()));

  info!("---");
  for i in yml.rel_lang_set("/")?.0 {
    info!("{} {:?}", i.0, i.1);
  }
  OK
}
