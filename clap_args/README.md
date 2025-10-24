# clap_args

```rust
use std::borrow::Borrow;

pub use clap::{self, ArgAction, arg};
use clap::{ArgMatches, Command};
pub use const_str;
use current_platform::CURRENT_PLATFORM;

pub fn parse(
  project: impl Into<String>,
  ver: impl Borrow<[u64; 3]>,
  cmd_build: impl FnOnce(Command) -> Command,
) -> Option<ArgMatches> {
  let mut cmd = cmd_build(
    Command::new(project.into())
      .disable_version_flag(true)
      .disable_help_flag(true)
      .arg(arg!(-v --version "show version").action(clap::ArgAction::SetTrue))
      .arg(arg!(
        --vv "version detail"
      ))
      .arg(arg!(
        -h --help "print help"
      )),
  );
  {
    let cmd2 = cmd.clone();
    let m = cmd2.ignore_errors(true).get_matches();
    if let Some(help) = m.get_one::<bool>("help")
      && *help
    {
      xerr::log!(cmd.print_help());
      return None;
    }

    let vv = m.get_one("vv") == Some(&true);
    if let Some(n) = m.get_one("version")
      && *n
    {
      let ver = ver.borrow();
      println!("{}.{}.{}", ver[0], ver[1], ver[2]);
      return None;
    } else if vv {
      let ver = ver.borrow();
      println!(
        r#"ver:{}.{}.{}
target:{CURRENT_PLATFORM}"#,
        ver[0], ver[1], ver[2]
      );
      return None;
    }
  }

  let matches = cmd.get_matches();
  Some(matches)
}

#[cfg(feature = "macro")]
#[macro_export]
macro_rules! parse {
  ($cmd_build: expr) => {{
    $crate::parse(
      env!("CARGO_PKG_NAME"),
      [
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
        $crate::const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
      ],
      $cmd_build,
    )
  }};
}
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
