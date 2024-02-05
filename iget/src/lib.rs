use std::{fmt::Display, sync::Arc, time::Duration};

use aok::{Result, OK};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::SliceRandom;
use reqwest::{header::RANGE, Client, ClientBuilder, StatusCode};
use thiserror::Error;
use tokio::{io::AsyncWriteExt, join, sync::RwLock, task::JoinHandle, time::timeout};

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

#[derive(Debug)]
pub enum UrlOrProgressBar {
    Url(String),
    ProgressBar(ProgressBar),
}

#[derive(Debug)]
pub struct Bar {
    pub total: u64,
    pub now: u64,
    pub pb: UrlOrProgressBar,
}

impl Bar {
    pub fn done(&self) {
        if let UrlOrProgressBar::ProgressBar(pb) = &self.pb {
            pb.finish();
        }
    }

    pub fn incr(&mut self, n: u64) {
        self.now += n;
        let now = self.now;

        let update_total = now > self.total;
        if update_total {
            self.total = now + MB16;
        }

        if let UrlOrProgressBar::ProgressBar(pb) = &self.pb {
            if update_total {
                pb.set_length(self.total);
            }
            pb.set_position(now);
        }
    }
}

pub struct _Down {
    pub bar: Arc<RwLock<Bar>>,
    pub ing: JoinHandle<Result<(), aok::Error>>,
}

pub struct Down(pub Option<_Down>);

impl Down {
    pub async fn show(self) -> Result<()> {
        if let Some(i) = self.0 {
            // {} 用来释放锁
            {
                let mut bar = i.bar.write().await;
                if let UrlOrProgressBar::Url(url) = &bar.pb {
                    let pb = { ProgressBar::new(bar.total) };
                    pb.set_style(ProgressStyle::default_bar()
          .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
          .progress_chars("─> "));
                    pb.set_message(url.to_string());
                    bar.pb = UrlOrProgressBar::ProgressBar(pb);
                }
            }
            let _ = i.ing.await?;
        }
        OK
    }
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

    pub async fn down(
        &self,
        url: impl AsRef<str> + Display,
        path: impl AsRef<str>,
    ) -> Result<Down> {
        let url = self.url.clone() + url.as_ref();
        let path = path.as_ref();

        let mut req = self.client.get(&url);

        let file_size = ifs::size(path).await;

        if file_size > 0 {
            req = req.header(RANGE, format!("bytes={}-{}", file_size, ""));
        }

        let mut res = req.send().await?;
        let mut status = res.status();

        if status == StatusCode::RANGE_NOT_SATISFIABLE {
            res = reqwest::get(&url).await?;
            status = res.status();
        }

        let content_len = res.content_length();
        dbg!((&url, &status, content_len));
        let mut file = if status == StatusCode::PARTIAL_CONTENT {
            ifs::append(path).await?
        } else if status == StatusCode::OK {
            if content_len == Some(file_size) {
                return Ok(Down(None));
            }
            ifs::w(path).await?
        } else {
            return Err(ReqError::Status(status, res.text().await?).into());
        };

        let bar = Arc::new(RwLock::new(Bar {
            now: 0,
            total: content_len.unwrap_or(MB16),
            pb: UrlOrProgressBar::Url(url.to_string()),
        }));

        Ok(Down(Some(_Down {
            bar: bar.clone(),
            ing: tokio::spawn(async move {
                let mut stream = res.bytes_stream();

                while let Some(chunk) = timeout(Duration::from_secs(60), stream.next()).await? {
                    let chunk = chunk?;
                    let (f, _) = join!(file.write_all(&chunk), async {
                        bar.write().await.incr(chunk.len() as _);
                    });
                    f?;
                }
                file.flush().await?;

                {
                    bar.read().await.done();
                }
                OK
            }),
        })))
    }
}
