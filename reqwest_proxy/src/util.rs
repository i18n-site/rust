use std::pin::Pin;

use bytes::Bytes;
use futures_util::{future::poll_fn, stream::Stream};
use http_body_util::Full;
use hyper::body::{Body, Incoming};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;

// Define the HyperClient type, using Full<Bytes> as the request body
// 定义 HyperClient 类型，使用 Full<Bytes> 作为请求体
pub(crate) type HyperClient<C> = Client<HttpsConnector<C>, Full<Bytes>>;

// Convert hyper's body to a stream of bytes
// 将 hyper 的 body 转换为字节流
pub(crate) fn body_to_stream(
  mut body: Incoming,
) -> impl Stream<Item = Result<Bytes, anyhow::Error>> {
  async_stream::try_stream! {
      while let Some(frame) = poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await {
          let frame = frame.map_err(|e| anyhow::anyhow!(e))?;
          if let Some(chunk) = frame.data_ref() {
              yield chunk.clone();
          }
      }
  }
}
