use std::{
  collections::{HashMap, HashSet},
  io::Write,
  path::Path,
};

use aok::Result;
use lang::Lang;
use prost::Message;
use redb::{Database, ReadableTable, TableDefinition};
use reqwest::StatusCode;
use speedy::{Readable, Writable};
use static_init::dynamic;
use thiserror::Error;
use tracing::{info, warn};

use crate::{api, api::Ext, lang_name_li, FromTo};

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
  token: &str,
  from: Lang,
  ext: &str,
  root: &Path,
  rel: &str,
  from_to: &FromTo,
  db: &Database,
) -> Result<Vec<u16>> {
  let gendir = root.join(".gen");

  let hashfp = gendir.join("hash").join(rel);
  let mut traned_lang = vec![];

  let mut from_set = HashSet::new();

  for from_lang in from_to.ft.keys().copied() {
    from_set.insert(from_lang);
    if from != from_lang {
      traned_lang.push(from_lang as _);
    }
  }

  let mut src_hash = HashMap::new();
  let mut to_tran = Vec::new();

  let lm = format!("{rel}>mtimeLen");
  let lm: TableDefinition<u16, (u64, u64)> = TableDefinition::new(&lm);
  let txn = db.begin_write()?;

  {
    let mut lm = txn.open_table(lm)?;

    macro_rules! lm_push {
      ($lang:expr,$len:expr, $mtime:expr) => {{
        lm.insert($lang, ($len, $mtime))?;
      }};
    }

    let mut traned_cache = if let Ok(pre) = ifs::r(&hashfp)
      && !pre.is_empty()
      && let Ok(mut traned) = xerr::ok!(TranedCache::read_from_buffer(&pre))
    {
      let keys = traned.m.keys().copied().collect::<Vec<_>>();

      for (lang, lang_name) in keys
        .iter()
        .zip(lang_name_li(&keys, &from_to.lang_str).await?)
      {
        let lang = *lang;
        let i = traned.m.get(&lang).unwrap();
        if let Ok(l) = lang.try_into() {
          from_set.remove(&l);
        }

        let fp = root.join(&lang_name).join(rel);
        // 检查文件修改时间，若相同则跳过
        if let Some((len, mtime)) = ifs::len_mtime(&fp) {
          if let Some(v) = lm.get(lang)?
            && let (pre_len, pre_mtime) = v.value()
            && pre_len == len
            && pre_mtime == mtime
          {
            info!("{rel} {:?} len mtime same", TryInto::<Lang>::try_into(lang));
            continue;
          }

          lm_push!(lang, len, mtime);

          if let Ok(bin) = ifs::r(fp) {
            // 检查文件内容是否已经相同，若不同则加入翻译列表
            let b3 = *blake3::hash(&bin).as_bytes();
            let len = bin.len() as u64;
            if i.len == len && i.b3 == b3 {
              continue;
            }

            src_hash
              .entry(i.from as u32)
              .or_insert_with(|| traned.src_hash.get(&i.from).unwrap().clone());

            to_tran.push((lang, bin));
            traned.m.insert(
              lang,
              TranedLang {
                from: i.from,
                len,
                b3,
              },
            );
          }
        }
      }
      traned
    } else {
      Default::default()
    };

    macro_rules! lmfp {
      ($lang:expr,$fp:expr) => {
        if let Some((len, mtime)) = ifs::len_mtime($fp) {
          lm_push!($lang, len, mtime);
        }
      };
    }

    for from_lang in from_set {
      let fp = root
        .join(from_to.lang_str.get(&from_lang).unwrap())
        .join(rel);
      let from_lang = from_lang as u16;
      if let Ok(bin) = std::fs::read(&fp) {
        traned_cache.m.insert(
          from_lang,
          TranedLang {
            from: from_lang,
            len: bin.len() as _,
            b3: *blake3::hash(&bin).as_bytes(),
          },
        );
        to_tran.push((from_lang as _, bin));
        lmfp!(from_lang, fp);
      }
    }

    if !to_tran.is_empty() {
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

      dbg!(&api_tran);
      let res = ireq::REQ
        .post(&*API_TRAN)
        .header("t", token)
        .body(api_tran.encode_to_vec())
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

        dbg!(&traned);
        for i in traned.src_hash.into_iter() {
          traned_cache.src_hash.insert(i.0 as _, i.1);
        }

        for (i, name) in traned.li.iter().zip(
          lang_name_li(
            &traned.li.iter().map(|i| i.lang as _).collect::<Vec<_>>(),
            &from_to.lang_str,
          )
          .await?,
        ) {
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
            lmfp!(lang, fp);
          }
        }
        ifs::w(&hashfp)?.write_all(&traned_cache.write_to_vec()?)?;
      } else {
        warn!("can't tran {rel}");
      }
    }
  }
  txn.commit()?;
  Ok(traned_lang)
}
