# ghapi

```rust
use std::env::var;

use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

use aok::{Result, OK};
// use static_init::constructor;
pub const FOLLOW_USER: &[&str] = &["i18n-site", "i18nsite", "i18n-api", "i18n-cron", "i18n-ops"];

pub const FOLLOW_REPO: &[(&str, &[&str])] = &[
  (
    "i18n-site",
    &[
      "site",
      "18x",
      "demo.i18n.site",
      "demo.i18n.site.docker",
      "lib",
      "font",
      "md",
      "plugin",
      "rust",
      "ie",
      "alive",
      "site.conf",
    ],
  ),
  ("i18n-api", &["pay_webhook", "pub", "srv", "srv.docker"]),
  ("i18n-cron", &["cron"]),
  ("i18n-ops", &["docker", "ops", "os", "ubuntu"]),
];

#[tokio::test]
async fn test() -> Result<()> {
  // let code = "x13";
  // let code = var("code").unwrap();
  // let r = ghapi::access_token(var("GITHUB_ID").unwrap(), var("GITHUB_SK").unwrap(), code).await?;
  //
  // println!("token: {:?}", r);
  // token_type: "bearer", scope: "public_repo,user:email,user:follow"
  // let token = r.access_token;

  let token_li = var("GITHUB_LI").unwrap();
  let token_li = token_li.split_whitespace();

  for (pos, token) in token_li.enumerate() {
    let token = format!("gho_{token}");
    // let token = "test";
    let user = ghapi::User::new(token);

    // user
    //   .star_user_repo(["zRzRzRzRzRzRzR"], ["THUDM/GLM-4"])
    //   .await?;
    user
      .star_user_repo(
        FOLLOW_USER,
        FOLLOW_REPO
          .iter()
          .flat_map(|(org, repo_li)| repo_li.iter().map(move |repo| format!("{org}/{repo}")))
          .collect::<Vec<_>>(),
      )
      .await?;

    info!("{}", pos);
    //   UserInfo { login: "i18nsite", id: 145643935, followers: 1, following: 8, created_at: "2023-09-21T05:54:58Z", updated_at: "2024-08-02T07:04:51Z", twitter_username: None, name: Some("i18n.site"), company: None }
    // [UserEmail { email: "i18n.site@gmail.com", primary: true }]
    //
    // user.star_repo("apache/kvrocks").await?;
    // println!("starred repo apache/kvrocks");
    //
    // user.follow("str4d").await?;
    // println!("followed user str4d");
  }

  OK
}
```

https://github.com/login/oauth/authorize?client_id=Ov23liuGAmK0plc9FgB3&scope=user:email,user:follow,public_repo

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