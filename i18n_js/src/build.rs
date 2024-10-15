use std::path::{Path, PathBuf};

use aok::{Null, Result, OK};
use ft::FromTo;
use futures::{stream, stream::StreamExt};
use globset::GlobSet;
use gxhash::{HashMap, HashMapExt, HashSet};
use lang::{Lang, LANG_CODE};
use verfs::VerFs;

use crate::{
  bjs_after, bjs_after::BjsAfter, css, index_html, mnt::Mnt, pug, worker::worker, Conf, HtmConf,
  NavLi, Scan, DOT_I18N, HTM, OUT, PUBLIC,
};

const DATA: &str = "data";
const V: &str = "v";
const REL_I18N_CONF: &str = "../.i18n/conf.yml";

#[derive(Debug)]
pub struct Build {
  pub bjs_after: BjsAfter,
  pub conf: Conf,
  pub css: String,
  pub htm: PathBuf,
  pub htm_conf: HtmConf,
  pub htm_conf_name: String,
  pub i18n: PathBuf,
  pub i18n_li: HashMap<Lang, Vec<String>>,
  pub lang_li: Vec<Lang>,
  pub mnt: Mnt,
  pub nav: String,
  pub pug: HashMap<String, crate::pug::Htm>,
  pub root: PathBuf,
  pub scan: change::Scan,
}

impl Build {
  pub fn foot(&self) -> HashMap<Lang, String> {
    let mut r = HashMap::with_capacity(self.lang_li.len());
    if let Some(foot) = self.pug.get("foot") {
      if foot.has_i18n {
        let extract = rvar::extract(&foot.htm);
        for (lang, li) in &self.i18n_li {
          let foot = extract.replace(&foot.htm, |key| {
            if let Some(key) = key.strip_prefix("I[") {
              if let Some(key) = key.strip_suffix(']') {
                if let Ok::<usize, _>(key) = key.parse() {
                  if key < li.len() {
                    return li[key].clone();
                  }
                }
              }
            }
            key.into()
          });

          r.insert(*lang, format!("<footer>{foot}</footer>"));
        }
      } else {
        for lang in &self.lang_li {
          r.insert(*lang, foot.htm.clone());
        }
      }
    }
    r
  }

  pub async fn new(
    root: impl Into<PathBuf>,
    conf: Conf,
    ignore: &GlobSet,
    htm_conf_name: impl Into<String>,
    js_dir: &Path,
    after_tran: &[PathBuf],
    changed: &HashSet<String>,
  ) -> Result<Self> {
    let htm_conf_name = htm_conf_name.into();
    let root = root.into();
    let i18n = root.join(DOT_I18N);
    let htm = i18n.join(HTM);
    let scan = Scan::new(&htm);
    let css = css::css(&htm, &scan.css_li)?;
    // let conf: Conf = yconf::load(&i18n.join("conf.yml"))?;
    let nav = NavLi::new(&conf.nav);
    let mut nav_i18n_li = nav.i18n_li();

    let pug = pug::pug(&htm, &scan.pug_li, &mut nav_i18n_li)?;

    let from_to = FromTo::from_iter(conf.i18n.fromTo.iter());

    let lang_li = from_to.root_all_lang_li();

    let bjs_after = bjs_after(&root, &lang_li, js_dir, after_tran, changed)?;

    let nav = nav.json()?;
    let mut i18n_li = HashMap::with_capacity(lang_li.len());

    for i in &lang_li {
      let i = *i;
      let dir = root.join(LANG_CODE[i as usize]);
      let mut m: HashMap<String, String> = serde_yaml::from_slice(&ifs::r(dir.join("i18n.yml"))?)?;
      let mut li = Vec::new();
      if !nav_i18n_li.0.is_empty() {
        for i in &nav_i18n_li.0 {
          li.push(m.remove(i).unwrap_or_default());
        }
      }
      i18n_li.insert(i, li);
    }

    let mnt = Mnt::load(
      &root,
      &conf.upload,
      ignore,
      // &nav.0,
      // &from_to,
      &lang_li,
    )?;

    let mut scan = change::Scan::new(root.join(PUBLIC))?;
    scan.add(REL_I18N_CONF)?;

    let htm_conf = yconf::load(&htm.join(format!("{}.yml", htm_conf_name)))?;

    Ok(Self {
      scan,
      htm_conf,
      htm_conf_name,
      i18n_li,
      lang_li,
      bjs_after,
      conf,
      css,
      root,
      i18n,
      htm,
      pug,
      nav,
      mnt,
    })
  }

  pub async fn build(&self, ver: Option<String>, htm_index_js: &str) -> Result<VerFs> {
    let root = &self.root;
    let conf_name = &self.htm_conf_name;
    let outdir = root.join(OUT).join(conf_name);
    let outv = outdir.join(V);
    let mut vfs = VerFs::load(
      root,
      outv,
      root
        .join(DOT_I18N)
        .join(DATA)
        .join(V)
        .join(conf_name)
        .join("v.hash"),
      ver,
    )?;

    if vfs.verdir.exists() {
      std::fs::remove_dir_all(&vfs.verdir)?;
    }

    let conf = &self.htm_conf;

    let js_ver = self.js(&mut vfs, conf_name, conf, htm_index_js).await?;
    let prefix_index_ver = self.mnt.build(&mut vfs, &self.bjs_after.lang_path_bin)?;
    if vfs.has_new() {
      let v = format!("{js_ver}>{prefix_index_ver}");
      for i in [vfs.verdir.join(".v"), vfs.out.join(".v")] {
        ifs::wbin(i, &v)?;
      }
    }

    Ok(vfs)
  }

  pub async fn htm(&self, kind: &str, upload: &impl ckv::Ckv, lang_li: &[Lang]) -> Null {
    let root = &self.root;
    let conf = &self.htm_conf;
    let conf_name = &self.htm_conf_name;
    let public = &self.scan.public;
    let change = self.scan.change(
      root
        .join(DOT_I18N)
        .join(DATA)
        .join(PUBLIC)
        .join(conf_name)
        .join(kind),
    )?;
    if change.has_change {
      let changed = &change.changed;
      // 有可能只是删除了文件
      if !changed.is_empty() {
        let mut bar = pbar::pbar(changed.len() as u64);
        let mut iter = stream::iter(changed.iter().map(|(rel, _)| async move {
          if rel == REL_I18N_CONF {
            worker(root, conf, upload).await?;
          } else if let Some(fp) = public.join(rel).as_os_str().to_str() {
            if rel == "index.html" {
              upload
                .put(rel, index_html(root, fp, lang_li)?.as_bytes())
                .await?;
            } else {
              upload.put_path(rel, fp).await?;
            }
          }
          Ok::<_, aok::Error>(rel)
        }))
        .buffer_unordered(64);

        while let Some(rel) = iter.next().await {
          let rel = rel?;
          bar.inc(1);
          bar.set_message(format!("⬆ {rel}"));
        }

        bar.finish_and_clear();
      }
      change.save()?;
    }

    OK
  }
}
