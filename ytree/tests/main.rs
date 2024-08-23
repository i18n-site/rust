use std::io::{BufRead, Cursor};

use aok::{Result, OK};
use lang::Lang;
use static_init::constructor;
use tracing::info;
use ytree::Li;

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

  for i in root.iter() {
    info!("{i}");
  }

  let yml = serde_yaml::to_string(&root).unwrap();
  info!("{}", yml);

  let yml = ytree::lang::dumps([(vec![Lang::Ja, Lang::En, Lang::Zh, Lang::ZhTw], root)]);

  info!("{yml}");
  let cursor = Cursor::new(yml.as_bytes());

  let yml = ytree::lang::loads(cursor.lines().filter_map(|i| i.ok()));

  info!("---");
  for i in yml.iter() {
    info!("{i}");
  }
  OK
}
