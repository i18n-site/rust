use std::{collections::HashMap, io::Write, path::Path};

use aok::Result;
use ft::FromTo;
use lang::LANG_CODE;
use mreq::Mreq;
use prost::Message;
use reqwest::StatusCode;
use speedy::{Readable, Writable};
use tracing::warn;

use crate::{api, api::Ext, lang_name_li, need_tran, need_tran::lmfp, Err, Table, API_TRAN};

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
  req: &mut Mreq,
  db: &Table<'a>,
  ext: &str,
  root: &Path,
  from_to: &FromTo,
) -> Result<(u64, u64)> {
  let to_tran = need_tran.to_tran;
  let rel = &need_tran.rel;

  let mut char_sum = 0;
  let mut lang_count = 0;

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
    let li = to_tran
      .iter()
      .map(|i| {
        let txt = String::from_utf8_lossy(&i.1);
        let lang: lang::Lang = i.0.try_into().unwrap();

        let to_lang_count = from_to.to_lang_count(lang);
        char_sum += txt.chars().count() * to_lang_count;
        lang_count += to_lang_count;

        api::File {
          lang: i.0 as _,
          txt: txt.into(),
        }
      })
      .collect();

    let api_tran = api::Tran {
      ext: match ext {
        "yml" => Ext::Yml,
        _ => Ext::Md,
      }
      .into(),
      from_to: ft,
      li,
      src_hash,
    };

    let body = api_tran.encode_to_vec();
    let bin = match req.post(API_TRAN, body).await {
      Ok(r) => r,
      Err(e) => {
        if let Some(e) = e.downcast_ref::<mreq::Error>() {
          if let mreq::Error::Status { code, .. } = e
            && *code == StatusCode::UNAUTHORIZED
          {
            return Err(Err::Token.into());
          }
        }
        return Err(e);
      }
    };

    // let status = res.status();
    // if status != StatusCode::OK {
    //   if status == StatusCode::UNAUTHORIZED {
    //     return Err(Err::Token.into());
    //   }
    //   return Err(Err::Tran(status.as_u16(), res.text().await?).into());
    // }

    let bin_len = bin.len();
    if bin_len > 0 {
      let tran_result = if 1 == bin_len {
        api::TranResult::default()
      } else {
        api::TranResult::decode(bin)?
      };
      if let Some(err) = tran_result.err {
        return Err(Err::Api(err).into());
      } else if let Some(traned) = tran_result.traned {
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
      }
    } else {
      warn!("❌ {rel}");
    }
  }
  Ok((char_sum as u64, lang_count as u64))
}
