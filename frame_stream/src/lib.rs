#![cfg_attr(docsrs, feature(doc_cfg))]

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
        framed_chunk.put_slice(chunk);

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
