use std::{collections::HashMap, io::Write, path::Path};

use aok::{Null, OK};
use ft::FromTo;
use prost::Message;
use redb::Database;
use reqwest::StatusCode;
use speedy::{Readable, Writable};
use static_init::dynamic;
use thiserror::Error;
use tracing::warn;

use crate::{api, api::Ext, lang_name_li, need_tran, need_tran::lmfp, Txn};

#[derive(Error, Debug)]
pub enum Error {
  #[error("api error {0} : {1}")]
  Api(u16, String),

  #[error("token error")]
  Token,
}

genv::def!(API:String | "https://s.i18n.site".to_owned());

#[dynamic]
pub static API_TRAN: String = format!("{}/tran", API());

#[derive(Readable, Writable, Debug, Clone)]
pub struct TranedLang {
  pub from: u16,
  pub len: u64,
  pub b3: [u8; 32],
}

#[derive(Readable, Writable, Default)]
pub struct TranedCache {
  pub m: HashMap<u16, TranedLang>,
  pub src_hash: HashMap<u16, Vec<u8>>,
}

pub async fn tran_path(
  need_tran: need_tran::NeedTran,
  token: &str,
  db: &Database,
  ext: &str,
  root: &Path,
  from_to: &FromTo,
) -> Null {
  let to_tran = need_tran.to_tran;
  let rel = &need_tran.rel;

  if !to_tran.is_empty() {
    let src_hash = need_tran.src_hash;
    let mut traned_cache = need_tran.cache;
    let api_tran = api::Tran {
      ext: match ext {
        "yml" => Ext::Yml,
        _ => Ext::Md,
      }
      .into(),
      from_to: from_to
        .ft
        .iter()
        .map(|(k, v)| api::FromTo {
          from: *k as _,
          to_li: v.iter().map(|i| *i as _).collect(),
        })
        .collect(),
      li: to_tran
        .iter()
        .map(|i| api::File {
          lang: i.0 as _,
          txt: String::from_utf8_lossy(&i.1).into(),
        })
        .collect(),
      src_hash,
    };

    let body = api_tran.encode_to_vec();
    let res = ireq::REQ
      .post(&*API_TRAN)
      .header("t", token)
      .body(body)
      .send()
      .await?;

    let status = res.status();
    if status != StatusCode::OK {
      if status == StatusCode::UNAUTHORIZED {
        return Err(Error::Token.into());
      }
      return Err(Error::Api(status.as_u16(), res.text().await?).into());
    }

    let bin = res.bytes().await?;
    let bin_len = bin.len();
    if bin_len > 0 {
      let traned = if 1 == bin_len {
        api::Traned::default()
      } else {
        api::Traned::decode(bin)?
      };

      let tx = Txn::open(db)?;
      let mut t_ml = tx.table(rel, ext)?;

      for i in traned.src_hash.into_iter() {
        // t_hash.insert(i.0 as u16, i.1)?;
        traned_cache.src_hash.insert(i.0 as _, i.1);
      }

      for (i, name) in traned.li.iter().zip(lang_name_li(
        traned.li.iter().map(|i| i.lang as _).collect::<Vec<_>>(),
        &from_to.lang_str,
      )) {
        let lang = i.lang as u16;
        if let Some(from) = from_to.from(lang) {
          let txt = i.txt.as_bytes();
          let len = i.txt.len() as u64;
          traned_cache.m.insert(
            lang,
            TranedLang {
              from: from as _,
              len,
              b3: *blake3::hash(txt).as_bytes(),
            },
          );
          let fp = &root.join(&name).join(rel);
          ifs::w(fp)?.write_all(txt)?;
          lmfp(lang, &mut t_ml, fp)?;
        }
      }
      ifs::w(need_tran.hashfp)?.write_all(&traned_cache.write_to_vec()?)?;
    } else {
      warn!("❌ {rel}");
    }
  }
  OK
}
