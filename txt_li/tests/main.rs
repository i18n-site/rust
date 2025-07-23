use aok::{OK, Void};
use tracing::info;
use txt_li::TxtLi;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_restore() -> Void {
  let mut txt_li = TxtLi::new();
  txt_li.push_tran("1");
  txt_li.push_no_tran("2");
  txt_li.push_tran("3");
  txt_li.push_no_tran("4");
  txt_li.push_md_line("## abc");
  txt_li.push_md_line("-");
  txt_li.push_md_line("  [");
  txt_li.push_md_line("  + -987");
  txt_li.push_md_line("******");
  txt_li.push_md_line("_____");
  txt_li.push_md_line("----");
  txt_li.push_md_line("-5+1");
  txt_li.push_md_line("- [x] efg");
  txt_li.push_md_line("- [ ] hlq");
  txt_li.push_md_line("- [ ]");
  txt_li.push_md_line("*. abc");
  txt_li.push_md_line("**abc**");
  txt_li.push_md_line("[ ]");
  txt_li.push_md_line("[ ] abc");
  txt_li.push_md_line("[^bignote]:");
  txt_li.push_md_line("[^bignote]:xyz");
  txt_li.push_md_line("1. ");
  txt_li.push_md_line("1.");
  txt_li.push_md_line("1. 测试");
  txt_li.push_md_line("| 表头1 | 表头2 |");
  txt_li.push_md_line("| <a> 表头1 | 表头2 |</a> |");
  txt_li.push_md_line(r"表头1 | 表头2 \| 123 | 表头3");
  txt_li.push_md_line(r"[![License](https://img.shields.io/crates/l/volo)](#license)");
  txt_li.push_md_line(r"![License](https://img.shields.io/crates/l/volo)");
  txt_li.push_md_line(r"![License]()");
  txt_li.push_md_line(r"![License](");
  txt_li.push_md_line(r"![](https://img.shields.io/crates/l/volo)");
  // dbg!(&txt_li.li);
  info!("{}", &txt_li.restore.load(&txt_li.li));

  OK
}
