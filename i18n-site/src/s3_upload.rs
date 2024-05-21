use std::sync::Arc;

use aok::{Null, OK};
use map_await::{MapAwait, StreamExt};
use s3_put::IntoByteStream;

use crate::{HashLen, Pg, SQL_BATCH_SIZE};

#[static_init::dynamic]
pub static S3: Arc<s3_put::S3Bucket> = Arc::new(s3_put::from_env());

pub fn urle(n: u64) -> String {
  burl::e(intbin::u64_bin(n))
}

pub struct Payload<T> {
  pub id: i64,
  pub name: String,
  pub data: T,
  pub hash_len: HashLen,
  pub mime: String,
}

pub async fn s3_upload<
  T: 'static + IntoByteStream + Send + Sync,
  I: IntoIterator<Item = Payload<T>>,
>(
  pg: &Pg,
  to_upload_fp: I,
) -> Null
where
  <I as IntoIterator>::IntoIter: Send + 'static,
{
  let mut iter = to_upload_fp.into_iter().map_unordered(16, move |payload| {
    let s3 = S3.clone();
    async move {
      let url = urle(payload.id as _);
      s3.put(&url, &payload.mime, payload.data).await?;
      println!("⇧ {} → {url}", payload.name);
      Ok::<_, aok::Error>(payload.id)
    }
  });

  let batch = *SQL_BATCH_SIZE;
  let mut id_li = Vec::with_capacity(batch);

  macro_rules! uploaded {
    () => {
      pg.execute("SELECT uploaded($1)", &[&id_li]).await?;
    };
  }

  while let Some(id) = iter.next().await {
    id_li.push(id?);
    if id_li.len() == batch {
      uploaded!();
      id_li.clear();
    }
  }

  if !id_li.is_empty() {
    uploaded!();
  }
  OK
}
