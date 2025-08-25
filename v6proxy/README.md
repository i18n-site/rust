# v6proxy

```rust
use std::{collections::HashMap, time::Duration};

use aok::Result;
use reqwest::{Client, Proxy};

genv::s!(
  IPV6_PROXY_USER,
  IPV6_PROXY_PASSWD,
  IPV6_PROXY_PORT:u16,
  IPV6_PROXY_IP_LI
);

const TIMEOUT: Duration = Duration::from_secs(60);

pub fn proxy(proxy: Proxy) -> reqwest::Client {
  Client::builder()
        .proxy(proxy)
        .zstd(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .danger_accept_invalid_certs(true)
        .connect_timeout(TIMEOUT).build().unwrap()
}

pub struct Host {
  pub name: String,
  pub client: reqwest::Client,
}

pub fn from_env() -> Result<Vec<Host>> {
  let url = format!("http://{}:{}@", *IPV6_PROXY_USER, *IPV6_PROXY_PASSWD,);
  let port: u16 = *IPV6_PROXY_PORT;

  let name_ip: HashMap<String, String> = sonic_rs::from_str(&IPV6_PROXY_IP_LI)?;

  let li = name_ip
    .into_iter()
    .map(|(name, ip)| {
      Ok(Host {
        name,
        client: proxy(reqwest::Proxy::https(format!("{url}{ip}:{port}"))?),
      })
    })
    .collect::<Result<_, aok::Error>>()?;

  Ok(li)
}

#[static_init::dynamic]
pub static HOST_LI: Vec<Host> = from_env().unwrap();
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
