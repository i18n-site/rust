use std::{
  io::{Seek, Write},
  path::PathBuf,
};

use kanal::{AsyncReceiver, AsyncSender};
use aok::{OK, Result};
use ireq::{REQ, reqwest::header};
use tracing::warn;
use bytes::Bytes;
use tokio::task::JoinHandle;

use crate::ChunkLi;

pub type Ing = JoinHandle<Result<()>>;

pub struct Runner {
  chunk_li: ChunkLi,
  ing: Vec<Ing>,
}

impl Runner {
  pub fn new(
    size: u64,
    path: impl Into<PathBuf>,
    info_send: AsyncSender<u64>,
    data_recv: AsyncReceiver<(u64, Bytes)>,
  ) -> Self {
    let path = path.into();
    let chunk_li = ChunkLi::new(size);

    let cl = chunk_li.clone();
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
            return OK;
          }
        }
      }
      OK
    });

    Self {
      ing: Vec::new(),
      chunk_li: cl,
    }
  }

  pub fn run(&mut self, url: ireq::reqwest::Url, send: &AsyncSender<(u64, Bytes)>) {
    let send = send.clone();
    let chunk_li = self.chunk_li.clone();
    self.ing.push(tokio::spawn(async move {
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
