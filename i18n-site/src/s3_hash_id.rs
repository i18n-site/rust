use std::fmt::Display;

use aok::{Null, Result, OK};
use s3_put::IntoByteStream;

use crate::{s3_upload, s3_upload::Payload, HashLen, Pg, SQL_BATCH_SIZE};

pub async fn s3_hash_id<
  Data: Send + Sync + IntoByteStream + 'static,
  Name: Sync + Display + Send + 'static,
  IntoIter: IntoIterator<Item = (HashLen, Data, Name)>,
>(
  pg: &Pg,
  site_id: i64,
  hash_data_li: IntoIter,
) -> Result<Vec<i64>> {
  let batch = *SQL_BATCH_SIZE;
  let mut hash_id = Vec::new();
  let mut to_upload_fp = vec![];
  let mut li = Vec::with_capacity(batch);
  let mut hash_li = Vec::with_capacity(batch);
  let mut len_li = Vec::with_capacity(batch);

  macro_rules! _s3_hash_id {
    () => {
      _s3_hash_id(
        pg,
        site_id,
        li,
        hash_li,
        len_li,
        &mut to_upload_fp,
        &mut hash_id,
      )
      .await?;
    };
  }
  for item in hash_data_li.into_iter() {
    hash_li.push(item.0 .0);
    len_li.push(item.0 .1);
    li.push((item.1, item.2));

    if li.len() == batch {
      _s3_hash_id!();
      li = Vec::with_capacity(batch);
      hash_li = Vec::with_capacity(batch);
      len_li = Vec::with_capacity(batch);
    }
  }

  if !li.is_empty() {
    _s3_hash_id!();
  }

  s3_upload(pg, to_upload_fp).await?;

  Ok(hash_id)
}

async fn _s3_hash_id<
  T: Send + Sync + IntoByteStream + 'static,
  Name: Sync + Display + Send + 'static,
>(
  pg: &Pg,
  site_id: i64,
  li: Vec<(T, Name)>,
  hash_li: Vec<[u8; 32]>,
  len_li: Vec<usize>,
  to_upload_fp: &mut Vec<Payload<T>>,
  hash_id: &mut Vec<i64>,
) -> Null {
  let sql = format!(
    "SELECT id,ts FROM upload_id_ts({site_id},ARRAY[{}],$1)",
    len_li
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<_>>()
      .join(","),
  );

  for ((((data, name), i), hash), len) in li
    .into_iter()
    .zip(
      pg.query(
        &sql,
        &[&hash_li], // &hash_li.iter().map(|i| &i[..]).collect::<Vec<_>>()[..],
      )
      .await?,
    )
    .zip(hash_li)
    .zip(len_li)
  {
    let id: i64 = i.get(0);
    let ts: i64 = i.get(1);
    let hash_len = (hash, len);
    if ts == 0 {
      let name = name.to_string();
      let mime = crate::mime(&name);
      to_upload_fp.push(Payload {
        id,
        name,
        data,
        hash_len,
        mime: mime.into(),
      });
    }
    hash_id.push(id);
  }
  OK
}
