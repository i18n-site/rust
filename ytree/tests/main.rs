use aok::{Result, OK};
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
    "x/news/begin.md",
  ];

  let mut root = Li(Vec::new());

  for path in paths {
    root.push(path);
  }

  let yaml = serde_yaml::to_string(&root).unwrap();
  println!("{}", yaml);
  let li: Li = serde_yaml::from_str(&yaml).unwrap();
  for i in paths.into_iter().chain(["x.md", "blog/news"]) {
    dbg!((i, li.contains(i)));
  }
  OK
}
