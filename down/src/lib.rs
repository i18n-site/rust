use aok::{OK, Result};
use ireq::{REQ, reqwest::IntoUrl};
use kanal::{AsyncReceiver, unbounded_async};
use tokio::spawn;
use tracing::warn;
mod chunk_li;
use chunk_li::ChunkLi;
mod runner;
use runner::Runner;

pub async fn meta(url: impl IntoUrl) -> Result<(u64, ireq::reqwest::Url)> {
  let res = REQ
    .get(url)
    .header("User-Agent", "curl/8.4.0")
    .send()
    .await?;
  let status = res.status();
  if ireq::SUCCESS_STATUS.contains(&status) {
    return Ok((res.content_length().unwrap_or(0), res.url().clone()));
  }
  Err(ireq::ReqError(res).into())
}

pub async fn down<U: IntoUrl>(
  url_li: impl IntoIterator<Item = U>,
  to_path: impl Into<std::path::PathBuf>,
) -> Result<AsyncReceiver<u64>> {
  let (send, recv) = kanal::bounded_async(1);

  let mut ing = Vec::new();
  for i in url_li.into_iter() {
    let i = i.into_url()?;
    let send = send.clone();
    ing.push(spawn(async move {
      match meta(i.clone()).await {
        Err(err) => warn!("{} : {err}", i.to_string()),
        Ok((size, url)) => {
          if size > 0 {
            send.send((url, size)).await?;
            drop(send);
          } else {
            warn!("{} filesize = 0", i.to_string());
          }
        }
      }
      OK
    }));
  }

  drop(send);
  let (data_send, data_recv) = unbounded_async();

  let (info_send, info_recv) = kanal::unbounded_async();
  if let Ok((first_url, filesize)) = recv.recv().await {
    info_send.send(filesize).await?;
    let mut runner = Runner::new(filesize, to_path, info_send, data_recv, || {
      for i in ing {
        i.abort();
      }
    });
    runner.run(first_url, &data_send);

    spawn(async move {
      while let Ok((url, size)) = recv.recv().await {
        if filesize == size {
          runner.run(url, &data_send);
        } else {
          warn!("{} filesize != {}", url.to_string(), size);
        }
      }
      drop(data_send);
    });
  }

  Ok(info_recv)
}
