use std::{fmt::Display, path::PathBuf, time::Duration};

use aok::{Result, OK};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::SliceRandom;
use reqwest::{header::RANGE, Client, ClientBuilder, StatusCode};
use thiserror::Error;
use tokio::{
  fs::{create_dir_all, File, OpenOptions},
  io::AsyncWriteExt,
  time::timeout,
};

#[derive(Debug)]
pub struct Site {
  pub url: String,
  pub client: Client,
}

#[derive(Error, Debug)]
pub enum ReqError {
  #[error("{0} {1}")]
  Status(StatusCode, String),
}

pub fn builder() -> ClientBuilder {
  Client::builder()
    .connect_timeout(Duration::from_secs(6))
    .timeout(Duration::from_secs(60))
}

#[static_init::dynamic]
pub static HTTP: Client = builder().build().unwrap();

#[static_init::dynamic]
pub static H3: Client = builder().http3_prior_knowledge().build().unwrap();

pub const MB16: u64 = 1048576 * 16;

impl<S: AsRef<str>> From<(bool, S)> for Site {
  fn from((h3, url): (bool, S)) -> Self {
    Self::new(h3, url.as_ref())
  }
}

macro_rules! req {
  ($name:ident, $func:ident, $rt:ty) => {
    pub async fn $name(&self, url: impl AsRef<str>) -> Result<$rt> {
      let url = self.url.clone() + url.as_ref();
      let req = self.client.get(&url);
      let res = req.send().await?;
      let status = res.status();
      let r = res.$func().await?;
      if status != StatusCode::OK {
        Err(ReqError::Status(status, format!("{url} {:?}", r)))?
      } else {
        Ok(r)
      }
    }
  };
}

impl Site {
  pub fn rand_new(prefix: impl Into<String>, li: &[(bool, impl AsRef<str>)]) -> Self {
    let (h3, site) = li.choose(&mut rand::thread_rng()).unwrap();
    let prefix = prefix.into();
    (*h3, prefix + site.as_ref()).into()
  }

  pub fn new(h3: bool, url: impl Into<String>) -> Self {
    Self {
      url: url.into(),
      client: if h3 { H3.clone() } else { HTTP.clone() },
    }
  }

  req!(txt, text, String);
  req!(bin, bytes, bytes::Bytes);

  pub async fn down(&self, url: impl AsRef<str> + Display, path: impl AsRef<str>) -> Result<()> {
    let url = &(self.url.clone() + url.as_ref());
    let path = path.as_ref();

    let mut req = self.client.get(url);
    if let Ok(meta) = tokio::fs::metadata(path).await {
      req = req.header(RANGE, format!("bytes={}-{}", meta.len(), ""));
    };

    let mut res = req.send().await?;
    let mut status = res.status();

    if status == StatusCode::RANGE_NOT_SATISFIABLE {
      res = reqwest::get(url).await?;
      status = res.status();
    }

    let pathbuf: PathBuf = path.into();
    if let Some(p) = pathbuf.parent() {
      create_dir_all(p).await?;
    }

    let mut file = if status == StatusCode::PARTIAL_CONTENT {
      OpenOptions::new().append(true).open(pathbuf).await?
    } else if status == StatusCode::OK {
      File::create(pathbuf).await?
    } else {
      return Err(ReqError::Status(status, res.text().await?).into());
    };

    let mut downloaded = 0;

    let pb = ProgressBar::new(res.content_length().unwrap_or(MB16));
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("─> "));
    pb.set_message(url.to_string());

    let mut stream = res.bytes_stream();

    while let Some(chunk) = timeout(Duration::from_secs(60), stream.next()).await? {
      let chunk = chunk?;
      file.write_all(&chunk).await?;

      downloaded += chunk.len() as u64;
      let len = pb.length().unwrap();
      if downloaded > len {
        pb.set_length(downloaded + MB16);
      }
      pb.set_position(downloaded);
    }

    pb.finish_with_message(format!("✅ {url}"));

    OK
  }
}
