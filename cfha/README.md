# cfapi 高可用 ( high availability )

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

genv::s!(CLOUDFLARE_EMAIL);
genv::s!(CLOUDFLARE_KEY);

const CONF: &str = r#"
me5.top: 
  u1: 
    - 184.174.36.122
    - 2a02:c206:2140:2465::1
  u2:
    - 184.174.34.189
    - 2a02:c206:2140:481::1
  u3:
    - 38.242.220.222
    - 2a02:c206:2139:9706::1
"#;

#[tokio::test]
async fn test() -> Result<()> {
  cfha::conf::yml(CONF)?;

  // let cfha = Cfha::new(&*CLOUDFLARE_KEY, &*CLOUDFLARE_EMAIL)?;
  // let host = "me5.top";
  // let ip = "2a02:c206:2140:481::1";
  // if let Some(zone) = cfha.zone(host).await? {
  //   let zone_id = zone.id;
  //   for i in cfha.record(&zone_id, host).await? {
  //     match i.content {
  //       DnsContent::AAAA { content } => {
  //         //2a02:c206:2140:481::1
  //         if content.to_string() == ip {
  //           // cfha.rm_record(&zone_id, &i.id).await?;
  //         }
  //       }
  //       DnsContent::A { content } => {
  //       }
  //       _ => {}
  //     }
  //   }
  //   cfha
  //     .add_a_record(
  //       &zone_id,
  //       host,
  //       DnsContent::AAAA {
  //         content: ip.parse()?,
  //       },
  //       true,
  //     )
  //     .await?;
  // }

  OK
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