Base on tokio postgres

Keep one postgres connection, will auto reconnect when connect close

基于 tokio postgres , 保留一个 postgres 连接 , 连接关闭时会自动重新连接

```rust
use pgw::{NONE, Pg, Sql};
use static_init::dynamic;
use tokio::time;

// get postgres connection uri from environment
#[dynamic]
static PG: Pg = Pg::from_env("PG_URL");

// prepared sql
#[dynamic]
static SQL_NSPNAME: Sql = PG.sql("SELECT oid FROM pg_catalog.pg_namespace LIMIT 2");

use tokio_postgres::types::Oid;

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  // dbg!(li().await?);
  for i in 0..2 {
    println!("loop {i}");
    match PG.q(&SQL_NSPNAME, &[]).await {
      Ok(li) => {
        for i in li {
          let oid: Oid = i.try_get(0).unwrap();
          dbg!(oid);
        }
      }
      Err(err) => {
        dbg!(err);
      }
    }
    let oid: Oid = PG
      .q00("SELECT oid FROM pg_catalog.pg_namespace LIMIT 1", NONE)
      .await?;
    dbg!(oid);
    time::sleep(std::time::Duration::from_secs(1)).await;
  }
  Ok(())
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
