use std::{
  io::{Seek, Write},
  path::PathBuf,
  sync::Arc,
};

use aok::{OK, Result};
use bytes::Bytes;
use ireq::{REQ, reqwest::header};
use kanal::{AsyncReceiver, AsyncSender};
use parking_lot::Mutex;
use tokio::task::JoinHandle;
use tracing::warn;

use crate::ChunkLi;

pub type Ing = JoinHandle<Result<()>>;

pub struct Runner {
  chunk_li: ChunkLi,
  ing: Arc<Mutex<Vec<Ing>>>,
}

impl Runner {
  pub fn new(
    size: u64,
    path: impl Into<PathBuf>,
    info_send: AsyncSender<u64>,
    data_recv: AsyncReceiver<(u64, Bytes)>,
    on_end: impl FnOnce() + Send + 'static,
  ) -> Self {
    let path = path.into();
    let chunk_li = ChunkLi::new(size);

    let ing = Arc::new(Mutex::new(Vec::new()));

    let this = Self {
      chunk_li: chunk_li.clone(),
      ing: ing.clone(),
    };

    tokio::spawn(async move {
      let mut file = std::fs::File::create(path)?;
      file.set_len(size)?;
      let mut downed = 0;
      while let Ok((begin, data)) = data_recv.recv().await {
        file.seek(std::io::SeekFrom::Start(begin))?;
        file.write_all(&data)?;
        if chunk_li.remove(begin).await {
          downed += data.len() as u64;
          info_send.send(downed).await?;
          if downed == size {
            let mut ing = ing.lock();
            while let Some(i) = ing.pop() {
              i.abort();
            }
            on_end();
            return OK;
          }
        }
      }
      OK
    });

    this
  }

  pub fn run(&mut self, url: ireq::reqwest::Url, send: &AsyncSender<(u64, Bytes)>) {
    let send = send.clone();
    let chunk_li = self.chunk_li.clone();
    self.ing.lock().push(tokio::spawn(async move {
      while let Some((begin, end)) = chunk_li.get().await {
        match ireq::req(
          REQ
            .get(url.clone())
            .header(header::RANGE, format!("bytes={begin}-{end}")),
        )
        .await
        {
          Ok(data) => {
            // dbg!((&url.host().unwrap().to_string(), begin));
            send.send((begin, data)).await?;
          }
          Err(err) => {
            warn!("❌ {url} : {err}");
          }
        };
      }
      OK
    }));
  }
}
