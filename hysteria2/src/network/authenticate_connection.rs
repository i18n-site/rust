use super::{
  build_auth_request::build_auth_request, generate_padding::generate_padding,
  validate_auth_response::validate_auth_response,
};
use crate::{HysteriaError, config::Config};

/// 执行认证握手
pub(crate) async fn authenticate_connection(
  conn: &quinn::Connection,
  config: &Config,
) -> Result<(), HysteriaError> {
  let (h3_conn, mut request_stream) =
    h3::client::new(h3_quinn::Connection::new(conn.clone())).await?;

  let padding = generate_padding();
  let req = build_auth_request(config, &padding)?;

  let mut stream = request_stream.send_request(req).await?;
  stream.finish().await?;

  let resp = stream.recv_response().await?;

  // Keep the H3 connection alive by spawning a task to drive it.
  // 保持 H3 连接活动，通过生成一个任务来驱动它。
  tokio::spawn(async move {
    let _ = h3_conn;
  });

  validate_auth_response(&resp)?;
  Ok(())
}
