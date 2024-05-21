use std::{collections::HashMap, io::Write, path::Path};

use aok::{Null, OK};
use ft::FromTo;
use lang::LANG_CODE;
use prost::Message;
use reqwest::StatusCode;
use speedy::{Readable, Writable};
use static_init::dynamic;
use tracing::warn;

use crate::{api, api::Ext, lang_name_li, need_tran, need_tran::lmfp, Err, Table};

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

pub async fn tran_path<'a>(
  need_tran: need_tran::NeedTran,
  token: &str,
  db: &Table<'a>,
  ext: &str,
  root: &Path,
  from_to: &FromTo,
) -> Null {
  let to_tran = need_tran.to_tran;
  let rel = &need_tran.rel;

  dbg!(rel);
  if !to_tran.is_empty() {
    let mut ft: Vec<_> = from_to
      .ft
      .iter()
      .map(|(k, v)| api::FromTo {
        from: *k as _,
        to_li: v.iter().map(|i| *i as _).collect(),
      })
      .collect();

    if let Some(default_from) = from_to.default_from {
      ft.push(api::FromTo {
        from: default_from as _,
        to_li: vec![],
      })
    }

    let src_hash = need_tran.src_hash;
    let mut traned_cache = need_tran.cache;
    let api_tran = api::Tran {
      ext: match ext {
        "yml" => Ext::Yml,
        _ => Ext::Md,
      }
      .into(),
      from_to: ft,
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
        return Err(Err::Token.into());
      }
      return Err(Err::Api(status.as_u16(), res.text().await?).into());
    }

    let bin = res.bytes().await?;
    let bin_len = bin.len();
    if bin_len > 0 {
      let traned = if 1 == bin_len {
        api::Traned::default()
      } else {
        api::Traned::decode(bin)?
      };

      for i in traned.err {
        let p = i.lang as usize;
        let lang = if p < LANG_CODE.len() {
          LANG_CODE[p].into()
        } else {
          format!("LANG_CODE {}", p)
        };
        eprintln!("❌ {} {} {} : {}", i.code, lang, rel, i.msg);
      }

      for i in traned.src_hash.into_iter() {
        // t_hash.insert(i.0 as u16, i.1)?;
        traned_cache.src_hash.insert(i.0 as _, i.1);
      }

      let mut batch = db.db.batch();
      for (i, name) in traned.li.iter().zip(lang_name_li(
        traned.li.iter().map(|i| i.lang as u16).collect::<Vec<_>>(),
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
          let fp = &root.join(name).join(rel);
          ifs::w(fp)?.write_all(txt)?;
          lmfp(lang, &mut batch, db, fp);
        }
      }
      batch.commit()?;
      ifs::w(need_tran.hashfp)?.write_all(&traned_cache.write_to_vec()?)?;
    } else {
      warn!("❌ {rel}");
    }
  }
  OK
}
