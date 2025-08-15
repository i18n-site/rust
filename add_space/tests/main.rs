use add_space::add_space;
use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  for (txt, exp) in [
    (
      "OAuth 2.0鉴权用户只能查询到通过OAuth 2.0鉴权创建的会议",
      "OAuth 2.0 鉴权用户只能查询到通过 OAuth 2.0 鉴权创建的会议",
    ),
    ("/* block comment */", "/* block comment */"),
    ("/* block 注释 */", "/* block 注释 */"),
    ("// line comment", "// line comment"),
    ("// line 注释", "// line 注释"),
    // from: test_end_space
    ("a ", "a "),
    ("a  ", "a  "),
    ("a啊 ", "a 啊 "),
    // from: test_newline_or_tab
    ("a\nb", "a\nb"),
    ("a\rb", "a\rb"),
    ("a\tb", "a\tb"),
    ("中\nb", "中\nb"),
    ("中\rb", "中\rb"),
    ("中\tb", "中\tb"),
    // from: test_spacing
    ("中文English", "中文 English"),
    ("中文English中文", "中文 English 中文"),
    ("中文123", "中文 123"),
    ("123中文", "123 中文"),
    ("中文!", "中文!"),
    ("中文?", "中文?"),
    ("价格是$50和¥300", "价格是 $50 和 ¥300"),
    ("价格是¥300", "价格是 ¥300"),
    (
      "当你凝视着bug，bug也凝视着你",
      "当你凝视着 bug，bug 也凝视着你",
    ),
    (
      "与PM战斗的人，应当小心自己不要成为PM",
      "与 PM 战斗的人，应当小心自己不要成为 PM",
    ),
    ("中文和拉丁字母English混排", "中文和拉丁字母 English 混排"),
    (
      "中文数字１２３４５６７８９０和半角数字1234567890混排",
      "中文数字１２３４５６７８９０和半角数字 1234567890 混排",
    ),
    (
      "使用了Python的print()函数打印\"你好,世界\"",
      "使用了 Python 的 print() 函数打印\"你好,世界\"",
    ),
    (
      "价格人民币¥100美元$100欧元€100英镑£100",
      "价格人民币 ¥100 美元 $100 欧元 €100 英镑 £100",
    ),
    ("全角空格　和半角空格 混用", "全角空格　和半角空格 混用"),
    (
      "AＡBＢCＣ和abc以及１２３和123混排",
      "AＡBＢCＣ 和 abc 以及１２３和 123 混排",
    ),
    ("文件保存在~/Documents目录", "文件保存在 ~/Documents 目录"),
    // from: test_symbols
    (
      "用户目录是~，完整路径是~/Documents",
      "用户目录是 ~，完整路径是 ~/Documents",
    ),
    ("函数add(a,b)返回a+b", "函数 add(a,b) 返回 a+b"),
    (
      "文件保存在/usr/local/bin/目录",
      "文件保存在 /usr/local/bin/ 目录",
    ),
    (
      "网址是example.com而不是example。com",
      "网址是 example.com 而不是 example。com",
    ),
    (r#"他说"这很好"然后离开了"#, r#"他说"这很好"然后离开了"#),
    (
      "安装命令是npm install --save-dev @types/react使用v16.8版本",
      "安装命令是 npm install --save-dev @types/react 使用 v16.8 版本",
    ),
    // ("价格是$50和¥300", "价格是 $50 和 ¥300"), // 重复
    (
      "name|age|gender表示不同字段",
      "name|age|gender 表示不同字段",
    ),
    (
      "5+3*2=11，需要满足x>0且y<100",
      "5+3*2=11，需要满足 x>0 且 y<100",
    ),
    (
      "命令是`ls -la`，注意不要用''",
      "命令是`ls -la`，注意不要用''",
    ),
    (r#"\t"#, r#"\t"#),
    // from: test_string_with_escapes
    ("你好\n world\t!", "你好\n world\t!"),
    (r#"你好\n world\t!"#, r#"你好\n world\t!"#),
    ("你好world", "你好 world"),
  ] {
    let add = add_space(txt);
    info!("{}", add);
    assert_eq!(add, exp);
  }
  info!(
    "{}",
    add_space("请参阅我们的[贡献指南](CONTRIBUTING.md)，了解如何上手的详细信息。")
  );

  OK
}
