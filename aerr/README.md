# aerr : error / result like anyhow for axum

```rust
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug)]
pub enum Err {
  Any(anyhow::Error),
  Response(Box<Response>),
}

#[derive(Debug)]
pub struct Error(pub Err);

pub type Result<T, E = Error> = anyhow::Result<T, E>;

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let err = self.0;
    match err {
      Err::Any(err) => {
        error!("{}\n{}", err.backtrace(), err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("ERR: {}", err)).into_response()
      }
      Err::Response(r) => *r,
    }
  }
}

impl<E> From<E> for Error
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(Err::Any(err.into()))
  }
}

pub fn none() -> Result<Response> {
  Ok((StatusCode::NO_CONTENT, b"").into_response())
}

pub fn ok(body: impl IntoResponse) -> Result<impl IntoResponse> {
  Ok(body.into_response())
}

pub fn err<T>(code: StatusCode, body: impl IntoResponse) -> Result<T, Error> {
  let mut res = body.into_response();
  *res.status_mut() = code;
  std::result::Result::Err(Error(Err::Response(Box::new(res))))?
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
