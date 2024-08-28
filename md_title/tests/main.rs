use aok::{Result, OK};
use md_title::md_title;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  for i in [
    " # 123  ",
    "# 456    ",
    "756",
    r#"
<h1 style="display:flex;justify-content:space-between">i18n.site ⋅ 国际化解决方案 <img src="//p.3ti.site/logo.svg" style="user-select:none;margin-top:-1px;width:42px"></h1>

命令行 Markdown 、 Yaml 翻译工具，帮您构建国际化的文档站，支持 [上百种语言](/i18/LANG_CODE)  …"#,
  ] {
    info!(">{}<", md_title(i));
  }
  OK
}
