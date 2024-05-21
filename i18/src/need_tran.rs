use std::{
  collections::{HashMap, HashSet},
  path::{Path, PathBuf},
};

use aok::Result;
use ft::FromTo;
use lang::{Lang, LANG_CODE};
use speedy::Readable;

use crate::{
  lang_name_li,
  tran_path::{TranedCache, TranedLang},
  Table, DOT_I18N,
};

pub fn lmfp(lang: u16, batch: &mut fjall::Batch, db: &Table, fp: &Path) {
  if let Some(i) = ifs::len_mtime(fp) {
    batch.insert(&db.table, lang.to_le_bytes(), vb::e([i.0, i.1]));
  }
}

pub struct NeedTran {
  pub rel: String,
  pub cache: TranedCache,
  pub to_tran: Vec<(u16, Vec<u8>)>,
  pub src_hash: HashMap<u32, Vec<u8>>,
  pub hashfp: PathBuf,
  pub len: u64,
}

pub fn need_tran(db: &Table, root: &Path, from_to: &FromTo, rel: String) -> Result<NeedTran> {
  let gendir = root.join(DOT_I18N);
  let hashfp = gendir.join("hash").join(&rel);
  let mut from_set: HashSet<Lang> = HashSet::from_iter(from_to.from_lang_li());
  let mut to_tran = vec![];
  let mut src_hash = HashMap::new();
  let mut total_len = 0;
  let mut file = 0;

  let mut miss_file = vec![];

  macro_rules! lm_push {
    ($lang:expr,$len:expr, $mtime:expr) => {{
      db.insert($lang.to_le_bytes(), vb::e([$len, $mtime]))?;
    }};
  }

  let mut need_tran = if let Ok(pre) = ifs::r(&hashfp)
    && !pre.is_empty()
    && let Ok(mut traned) = xerr::ok!(TranedCache::read_from_buffer(&pre))
  {
    let keys = traned.m.keys().copied().collect::<Vec<_>>();

    for (lang, lang_name) in keys.iter().zip(lang_name_li(&keys)) {
      let lang = *lang;
      let i = traned.m.get(&lang).unwrap();

      let mut is_from_lang = false;
      if let Ok(l) = lang.try_into() {
        is_from_lang = from_set.remove(&l);
        // if is_from_lang {
        //   for i in from_to.to_li(l) {
        //
        //   }
        // }
      }

      let fp = root.join(lang_name).join(&rel);
      // 检查文件修改时间，若相同则跳过
      if let Some((len, mtime)) = ifs::len_mtime(&fp) {
        if let Some(v) = db.get(lang.to_le_bytes())? {
          let lm = vb::d(v)?;
          if lm.len() >= 2 && lm[0] == len && lm[1] == mtime {
            // tracing::info!("{rel} {:?} len mtime same", TryInto::<Lang>::try_into(lang));
            continue;
          }
        }

        lm_push!(lang, len, mtime);

        if let Ok(bin) = ifs::r(fp) {
          // 检查文件内容是否相同，若不同则加入翻译列表
          let b3 = *blake3::hash(&bin).as_bytes();
          let len = bin.len() as u64;
          if i.len == len && i.b3 == b3 {
            // tracing::info!("{rel} {:?} hash same", TryInto::<Lang>::try_into(lang));
            continue;
          }

          src_hash
            .entry(i.from as _)
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
        if is_from_lang {
          total_len += len;
          file += 1;
        }
      } else {
        miss_file.push(lang);
      }
    }
    traned
  } else {
    Default::default()
  };

  for lang in miss_file {
    let exist = to_tran.iter().map(|i| i.0).collect::<HashSet<_>>();
    if let Some(f) = from_to.from(lang) {
      if !exist.contains(&(f as u16)) {
        from_set.insert(f);
      }
    }
  }

  let mut batch = db.db.batch();

  if !from_set.is_empty() {
    for lang in from_set.iter() {
      let fp = root.join(LANG_CODE[*lang as usize]).join(&rel);
      let from_lang = *lang as u16;
      if fp.exists() {
        if let Ok(bin) = xerr::ok!(std::fs::read(&fp)) {
          total_len += bin.len() as u64;
          file += 1;
          need_tran.m.insert(
            from_lang,
            TranedLang {
              from: from_lang,
              len: bin.len() as _,
              b3: *blake3::hash(&bin).as_bytes(),
            },
          );
          to_tran.push((from_lang as _, bin));
          lmfp(from_lang, &mut batch, db, &fp);
        }
      }
    }
  }

  batch.commit()?;

  Ok(NeedTran {
    len: if file > 0 { total_len / file } else { 0 },
    rel,
    cache: need_tran,
    src_hash,
    to_tran,
    hashfp,
  })
}
