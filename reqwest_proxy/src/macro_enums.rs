#[macro_export]
macro_rules! enums {

  ($($name:ident, $name_str:literal);+) => {
    pastey::paste!{
      $crate::enums!(
        $(
          $name, $name_str, [< $name:camel >]
        );+
      );
    }
  };

  ($($name:ident, $name_str:literal, $name_camel:ident);+) => {
    use std::{io, pin::Pin, task, task::Poll};
    use async_trait::async_trait;
    use hyper::Uri;
    use hyper_util::client::legacy::connect::Connection;
    use reqwest::{Request, Response};
    use reqwest_middleware::{Error, Middleware, Next};
    use tower::Service;
    use $crate::{Error::UnsupportedProtocol, ProxyMiddleware};

    $(
      #[cfg(feature = $name_str)]
      use $crate::$name;
    )+

    pub enum StreamEnum {
      $(
        #[cfg(feature = $name_str)]
        $name_camel($name::StreamEnumType)
      ),+
    }

    $(
      #[cfg(feature = $name_str)]
      impl From<$name::StreamType> for StreamEnum {
        fn from(s: $name::StreamType) -> Self {
          StreamEnum::$name_camel(s.into())
        }
      }
    )+

      #[derive(Clone)]
      pub enum ProxyConn {
        $(
          #[cfg(feature = $name_str)]
          $name_camel($name::Conn),
        )+
      }

    impl ProxyConn {
      pub fn from_url(url: &str) -> $crate::Result<Self> {
        if let Some(scheme) = url.split(':').next() {
          $(
            #[cfg(feature = $name_str)]
            if scheme == $name::SCHEME {
              let connector = $name::Conn::new(url)?;
              return Ok(ProxyConn::$name_camel(connector));
            }
          )+
        }
        Err(UnsupportedProtocol(url.into()))
      }
    }

    impl hyper::rt::Read for StreamEnum {
      fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: hyper::rt::ReadBufCursor<'_>,
      ) -> Poll<io::Result<()>> {
          match self.get_mut() {
            $(
              #[cfg(feature = $name_str)]
              StreamEnum::$name_camel(s) => Pin::new(s).poll_read(cx, buf),
            )+
          }
      }
    }

    impl hyper::rt::Write for StreamEnum {
      fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
      ) -> Poll<Result<usize, io::Error>> {
          match self.get_mut() {
            $(
              #[cfg(feature = $name_str)]
              StreamEnum::$name_camel(s) => Pin::new(s).poll_write(cx, buf),
            )+
          }
      }

      fn poll_flush(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Result<(), io::Error>> {
          match self.get_mut() {
            $(
              #[cfg(feature = $name_str)]
              StreamEnum::$name_camel(s) => Pin::new(s).poll_flush(cx),
            )+
          }
      }

      fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
      ) -> Poll<Result<(), io::Error>> {
          match self.get_mut() {
            $(
              #[cfg(feature = $name_str)]
              StreamEnum::$name_camel(s) => Pin::new(s).poll_shutdown(cx),
            )+
          }
      }
    }

    impl Connection for StreamEnum {
      fn connected(&self) -> hyper_util::client::legacy::connect::Connected {
          match self {
            $(
              #[cfg(feature = $name_str)]
              StreamEnum::$name_camel(s) => s.connected(),
            )+
          }
      }
    }

    pub enum ConnFuture {
      $(
        #[cfg(feature = $name_str)]
        $name_camel($crate::conn::$name::ConnFuture),
      )+
    }

    impl Future for ConnFuture {
      type Output = Result<StreamEnum, io::Error>;

      fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
          $(
            #[cfg(feature = $name_str)]
            ConnFuture::$name_camel(f) => Pin::new(f).poll(cx).map(|r| r.map(|s| s.into())),
          )+
        }
      }
    }
    impl Service<Uri> for ProxyConn {
      type Response = StreamEnum;
      type Error = io::Error;
      type Future = ConnFuture;

      fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self {
          $(
            #[cfg(feature = $name_str)]
            ProxyConn::$name_camel(c) => c.poll_ready(cx)
          ),+
        }
      }

      fn call(&mut self, uri: Uri) -> Self::Future {
        match self {
          $(
              #[cfg(feature = $name_str)]
              ProxyConn::$name_camel(c) => ConnFuture::$name_camel(c.call(uri))
          ),+
        }
      }
    }

    pub enum Proxy {
      $(
        #[cfg(feature = $name_str)]
        $name_camel(ProxyMiddleware<$name::Conn>),
      )+
    }

    #[async_trait]
    impl Middleware for Proxy {
      async fn handle(
          &self,
          req: Request,
          extensions: &mut http::Extensions,
          next: Next<'_>,
      ) -> Result<Response, Error> {
          match self {
            $(
              #[cfg(feature = $name_str)]
              Proxy::$name_camel(mw) => mw.handle(req, extensions, next).await,
            )+
          }
      }
    }

    impl Proxy {
      pub fn from_url(url: &str) -> $crate::Result<Self> {
        if let Some(init) = url.split(":").next() {
          $(
            #[cfg(feature = $name_str)]
            if init == $name::SCHEME {
                return Ok(Proxy::$name_camel($name::from_url(url)?));
            }
          )+
        }
        Err(UnsupportedProtocol(url.to_string()))
      }
    }
  };
}
