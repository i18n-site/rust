# frame_stream

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(doc_cfg)]

use std::convert::Infallible;

use bytes::{BufMut, Bytes, BytesMut};
use futures_lite::stream::{Stream, unfold};
use kanal::AsyncReceiver;

pub fn frame_stream<B: AsRef<[u8]>>(
  receiver: AsyncReceiver<B>,
) -> impl Stream<Item = Result<Bytes, Infallible>> {
  unfold(receiver, |rx| async move {
    match rx.recv().await {
      Ok(chunk) => {
        let chunk = chunk.as_ref();
        let len = chunk.len();
        let mut framed_chunk = BytesMut::with_capacity(4 + len);
        framed_chunk.put_u32_le(len as u32); // 4 字节长度前缀
        framed_chunk.put_slice(&chunk);

        let item = Ok(framed_chunk.freeze());
        let next_state = rx;
        Some((item, next_state))
      }
      Err(_) => None,
    }
  })
}

#[cfg(feature = "axum")]
pub fn response<B: AsRef<[u8]> + Send + 'static>(
  receiver: AsyncReceiver<B>,
) -> axum::response::Response {
  use axum::{body::Body, http::header::CONTENT_TYPE};

  axum::response::Response::builder()
    .header(CONTENT_TYPE, "application/octet-stream")
    .body(Body::from_stream(frame_stream(receiver)))
    .unwrap()
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
