#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]
use std::{
  cmp::min,
  io::{Seek, Write},
  path::Path,
  sync::Arc,
};

use tracing::{error, warn};
use aok::{OK, Result, Void};
use indexmap::IndexSet;
use ireq::{
  REQ,
  reqwest::{IntoUrl, header},
};
use kanal::bounded_async;
use roaring::treemap::RoaringTreemap;
use tokio::sync::Mutex;

pub const CHUNK_SIZE: u64 = 256 * 1024; // 256KB

pub async fn down<U: IntoUrl>(
  filesize: u64,
  url_li: impl IntoIterator<Item = U>,
  to_path: impl AsRef<Path>,
) -> Void {
  let url_li_len = url_li.len();

  if url_li_len == 0 {
    warn!("{to_path} no url for down");
    return OK;
  }

  let mut file = std::fs::File::create(to_path)?;
  file.set_len(filesize)?;

  let chunk_size = if url_li_li * CHUNK_SIZE > filesize {
    filesize.div_ceil(url_li_len)
  } else {
    CHUNK_SIZE
  };

  let total_chunk = filesize.div_ceil(chunk_size);

  let task_set = Arc::new(Mutex::new(
    IndexSet::<_, std::hash::RandomState>::from_iter(0..total_chunk),
  ));

  let (send, recv) = bounded_async(16);

  for url in url_li.into_iter() {
    let send = send.clone();
    let url = url.into_url()?;
    let task_set = task_set.clone();
    tokio::spawn(async move {
      let mut task_id;
      loop {
        {
          let mut li = task_set.lock().await;
          if let Some(id) = li.shift_remove_index(0) {
            task_id = id;
            li.insert(id);
          } else {
            break;
          }
        };
        let begin = task_id * chunk_size;
        let end = min(begin + chunk_size, filesize) - 1;

        match ireq::req(
          REQ
            .get(url.clone())
            .header(header::RANGE, format!("bytes={begin}-{end}")),
        )
        .await
        {
          Ok(data) => {
            {
              let mut set = task_set.lock().await;
              set.shift_remove(&task_id);
            };

            tracing::info!("{task_id} {}", url.host().unwrap().to_string());
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let _ = send.send((task_id, data)).await;
          }
          Err(err) => {
            if let Some(err) = err.downcast_ref::<ireq::ReqError>() {
              error!("❌ {err}")
            } else {
              error!("❌ {url} : {err}");
            };
          }
        }
      }
      OK
    });
  }

  drop(send);

  let mut ing = RoaringTreemap::from_iter(0..total_chunk);
  while let Ok((id, data)) = recv.recv().await {
    if ing.contains(id) {
      file.seek(std::io::SeekFrom::Start(id * chunk_size))?;
      file.write_all(&data)?;

      ing.remove(id);
      if ing.is_empty() {
        break;
      }
    }
  }

  OK
}

pub async fn meta(url: &str) -> Result<(u64, ireq::reqwest::Url)> {
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
