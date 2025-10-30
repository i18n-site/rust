use aok::{OK, Void};
use tracing::info;
use txt_li::TxtLi;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_restore() -> Void {
  // txt_li.push_md_line("-");
  // txt_li.push_md_line("  [");
  // txt_li.push_md_line("  + -987");
  // txt_li.push_md_line("-5+1");
  // txt_li.push_md_line("- [x] efg");
  // txt_li.push_md_line("- [ ] hlq");
  // txt_li.push_md_line("- [ ]");
  // txt_li.push_md_line("[ ]");
  // txt_li.push_md_line("[ ] abc");
  // txt_li.push_md_line("[^bignote]:");
  // txt_li.push_md_line("[^bignote]:xyz");
  // txt_li.push_md_line("| 表头1 | 表头2 |");
  // txt_li.push_md_line("| <a> 表头1 | 表头2 |</a> |");
  // txt_li.push_md_line(r"表头1 | 表头2 \| 123 | 表头3");
  // txt_li.push_md_line(r"[![License](https://img.shields.io/crates/l/volo)](#license)");
  // txt_li.push_md_line(r"![License](https://img.shields.io/crates/l/volo)");
  // txt_li.push_md_line(r"![License]()");
  // txt_li.push_md_line(r"![License](");
  // txt_li.push_md_line(r"![](https://img.shields.io/crates/l/volo)");
  // txt_li.push_md_line(r"[1]: https://img.shields.io");
  // txt_li.push_md_line(r"[1] https://img.shields.io");
  // txt_li.push_md_line(r"`测试一下`");
  // txt_li.push_md_line(r"<code>测试一下</code>");
  // txt_li.push_md_line(r"***");
  // txt_li.push_md_line(r"**");
  // txt_li.push_md_line(r"*");
  // txt_li.push_md_line(r"[SurrealDB-url]: https://surrealdb.com/");
  // txt_li.push_md_line(r"<!-- 测试 -->");
  // txt_li.push_md_line(
  //   r"- 🌐 **abc No Vendor Lock-in**: Switch providers, deploy anywhere, own your data",
  // );
  // txt_li.push_md_line(r"[![Forks][forks-shield]][forks-url]");
  // txt_li.push_md_line(r"**Cost**");
  // txt_li.push_md_line(r"- [x] **Cost**");
  // txt_li.push_md_line(r"·");
  // txt_li.push_md_line(" (");
  // txt_li.push_md_line("(");
  // txt_li.push_md_line("()");
  // dbg!(&txt_li.li);

  for i in [
    // "- a",
    // "+ b",
    // "+",
    // "1. ",
    // "1.",
    // "1. 测试",
    // "1",
    // r"<strong>Checkout our website »</strong>",
    // "**[Discord Server](https://discord.gg/37XJPXfz2w)** - Get help, share ideas, and connect with other users",
    // "Recently Completed ✅",
    // "✅ Recently Completed",
    // "**[🚀 Deployment](docs/deployment/index.md)** - Complete deployment guides for all scenarios",
    // "*. abc",
    // "** abc123 **",
    // "** abc456 **",
    // "***",
    // "**",
    // "*",
    // r"**Cost**:",
    // r"**Cost** - cost good",
    // r"- [x] **Cost**",
    // r"- 🌐 **abc No Vendor Lock-in**: Switch providers, deploy anywhere, own your data",
    // r"[SurrealDB-url]: https://surrealdb.com/",
    // r"[12] https://x.com/",
    // "<p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>",
    // r"[![License](https://img.shields.io/crates/l/volo)](#license)",
    // r"2<3",
    // "******",
    // "_____",
    // "----",
    // "## abc",
    "<p align=\"right\"></p>",
    "<div class=a>",
    // "🎙️ **Better Podcasts**: Full script control and multi-speaker flexibility vs limited 2-speaker deep-dive format",
    "<br />测试",
    "测1<br />测2",
    "测2<br>测4",
    "================",
  ] {
    let mut txt_li = TxtLi::new();
    txt_li.push_md_line(i);
    for i in &txt_li.li {
      info!("{:?}", i);
    }
    assert_eq!(i.trim_end(), txt_li.restore.load(&txt_li.li).trim_end());
  }
  OK
}
