use std::fs;

use aok::{OK, Result, Void};
use tracing::info;
use tran_fmt::tran_fmt;
use txt_li::TxtLi;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

struct P;

impl hugo_head::Parse for P {
  fn parse<I: IntoIterator<Item = S>, S: Into<String>>(txt_li: &mut TxtLi, iter: I) -> Void {
    let mut iter = iter.into_iter();
    if let Some(first_line) = iter.next() {
      for i in [first_line].into_iter().chain(iter) {
        let i = i.into();
        txt_li.push_tran_line(i);
      }
      // 不要新增回车
      if let Some((p, t)) = txt_li.restore.li.pop() {
        let t = t.trim_end();
        if !t.is_empty() {
          txt_li.restore.li.push((p, t.into()));
        }
      }
    };
    OK
  }
}

fn parse(s: &str) -> Result<TxtLi> {
  let li = tran_fmt(s).1;
  let txt_li = hugo_head::parse::<P, _>(li)?;
  Ok(txt_li)
}

#[test]
fn test() -> Void {
  for entry in fs::read_dir("tests/md")? {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
      info!("{}", path.display());
      let content = fs::read_to_string(&path)?;
      let r = parse(&content)?;
      info!("{}", r.restore.load(&r.li));

      info!("{:?}", r.li);
    }
  }
  OK
}
