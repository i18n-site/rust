use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
  path::{Path, PathBuf},
};

use aok::Result;
use globset::GlobSet;
use ifs::unix_path;
use lang::{Lang, LANG_CODE};
use sonic_rs::to_string;
use ver_count::VerCount;
use verfs::{PrefixLi, VerFs};
use walkdir::WalkDir;

use crate::conf::Upload;

pub fn ver(fp: &Path) -> Result<String> {
  let file = File::open(fp)?;
  let reader = BufReader::new(file);
  for line in reader.lines().map_while(Result::ok) {
    let line = line.trim();
    if !line.is_empty() {
      return Ok(line.into());
    }
  }
  Ok("".into())
}

#[derive(Debug)]
pub struct Mnt {
  lang_rel_li: Vec<(Lang, Vec<String>)>,
  prefix_ver: HashMap<String, String>,
}

impl Mnt {
  pub fn load(
    root: &Path,
    upload: &Upload,
    ignore: &GlobSet,
    // nav: &[Nav],
    // from_to: &ft::FromTo,
    lang_li: Vec<Lang>,
  ) -> Result<Self> {
    let mut prefix_ver = HashMap::<String, _>::new();

    let mut lang_rel_li = Vec::with_capacity(lang_li.len());
    for lang in lang_li.iter() {
      let code = LANG_CODE[*lang as usize];
      let dir = root.join(code);
      let mut rel_li = Vec::new();

      for entry in WalkDir::new(&dir).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          if entry.file_type().is_file() {
            let path = entry.path();
            let lang_rel = path.strip_prefix(root).unwrap();
            let lang_rel = unix_path(lang_rel.as_os_str().to_str().unwrap());

            let rel = lang_rel[code.len() + 1..].to_owned();

            if ignore.is_match(format!("/{rel}")) {
              continue;
            }

            if rel.ends_with("/v") {
              prefix_ver.insert(rel[..rel.len() - 2].into(), ver(path)?);
              continue;
            }

            if let Some(ext) = path.extension() {
              if !upload.ext.contains(&ext.to_str().unwrap().to_owned()) {
                continue;
              }
            } else {
              continue;
            }

            rel_li.push(rel);
          }
        }
      }
      rel_li.sort();
      lang_rel_li.push((*lang, rel_li));
    }

    Ok(Self {
      lang_rel_li,
      prefix_ver,
    })
  }

  pub fn build(&self, vfs: &mut VerFs, lang_path_bin: &crate::bjs_after::Lpb) -> Result<Box<str>> {
    let mut prefix_li: Vec<String> = self.prefix_ver.iter().map(|i| i.0.clone()).collect();
    prefix_li.push("".into());
    let pl = PrefixLi::new(prefix_li);

    let mut prefix_ver_id = HashMap::<_, VerCount>::new();

    for (lang_enum, file_li) in &self.lang_rel_li {
      let mut pl = pl.clone();
      let lang = LANG_CODE[*lang_enum as usize];

      for from in file_li {
        let to;
        let p = if let Some(p) = from.rfind("/") {
          p + 1
        } else {
          0
        };

        #[allow(clippy::never_loop)]
        loop {
          if p < from.len() {
            let end = from[p..].to_lowercase();
            if end == "readme.md" {
              to = from[..if p > 0 { p - 1 } else { p }].to_owned() + ".md";
              break;
            }
          }
          to = from.clone();
          break;
        }
        let ver = vfs.cp(format!("{lang}/{from}"), format!("{lang}/{to}"))?;
        pl.add(to, ver);
      }

      if let Some(path_bin) = lang_path_bin.get(lang_enum) {
        for (path, bin) in path_bin {
          let ver = vfs.wbin(format!("{lang}/{path}"), bin)?;
          pl.add(path, ver);
        }
      }
      for (prefix, trie) in pl.0 {
        let trie = to_string(&trie)?;
        let ver = vfs.wstr(&format!("{lang}/{prefix}.json"), trie)?;
        prefix_ver_id.entry(prefix).or_default().push(ver);
      }
    }

    let i18n_v: PathBuf = vfs.log.parent().unwrap().into();
    let prefix_li = pl.0.into_iter().map(|i| i.0).collect::<Vec<_>>();
    let mut prefix_info = Vec::with_capacity(prefix_li.len());
    for prefix in prefix_li {
      let map = prefix_ver_id.remove(&prefix).unwrap().map();
      let mut lang_pos = HashMap::new();
      let ver_li = map.li;
      for (lp, vp) in map.pos {
        let lang = self.lang_rel_li[lp].0;
        lang_pos.insert(LANG_CODE[lang as usize], vp);
      }
      let json = to_string(&(ver_li, lang_pos))?;
      // println!("{json}");
      let index_ver = vfs.wstr(&format!("{prefix}.js"), json)?;
      let mut li = vec![prefix.clone(), index_ver.clone().into()];
      if let Some(pv) = self.prefix_ver.get(&prefix).cloned() {
        if !pv.is_empty() {
          let vli = yml_dict_li::set(i18n_v.join(format!("{prefix}.yml")), &pv, index_ver)?;
          li.push(pv);
          if vli.len() > 1 {
            li.push(
              vfs
                .wstr(format!("{prefix}.v.json"), to_string(&vli)?)?
                .into(),
            );
          }
        }
      }
      prefix_info.push(li);
    }

    let prefix_index_ver = vfs.wstr("P.js", to_string(&prefix_info)?)?;
    Ok(prefix_index_ver)
  }
}
