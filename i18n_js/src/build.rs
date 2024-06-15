use std::{
  collections::{HashMap, HashSet},
  fs,
  path::PathBuf,
};

use aok::Result;
use ft::FromTo;
use globset::GlobSet;
use lang::{Lang, LANG_CODE};
use verfs::VerFs;
use walkdir::WalkDir;

use crate::{
  bjs_after, bjs_after::BjsAfter, css, mnt::Mnt, pug, worker, Conf, HtmConf, NavLi, Scan, DOT_I18N,
  HTM, PUBLIC,
};

const V: &str = "v";

#[derive(Debug)]
pub struct Build {
  pub root: PathBuf,
  pub htm: PathBuf,
  pub i18n: PathBuf,
  pub css: String,
  pub conf: Conf,
  pub nav: String,
  pub pug: String,
  pub mnt: Mnt,
  pub bjs_after: BjsAfter,
  pub index_js: String,
  pub lang: Vec<(Lang, Vec<u8>)>,
}

impl Build {
  pub fn new(root: impl Into<PathBuf>, conf: Conf, ignore: &GlobSet) -> Result<Self> {
    let root = root.into();
    let i18n = root.join(DOT_I18N);
    let htm = i18n.join(HTM);
    let scan = Scan::new(&htm);
    let css = css::css(&htm, &scan.css_li)?;
    // let conf: Conf = yconf::load(&i18n.join("conf.yml"))?;
    let nav = NavLi::new(&conf.nav);
    let mut i18n_li = nav.i18n_li();

    let pug = pug::pug(&htm, &scan.pug_li, &mut i18n_li)?;
    let pug = format!(
      "{{{}}}",
      pug.iter().map(|i| i.to_fn()).collect::<Vec<_>>().join(",")
    );

    let fp = htm.join("index.js");
    let index_js = minjs::file(&fp)?;

    let from_to: FromTo = (&conf.i18n.fromTo).into();

    let lang_li = from_to.all_lang_li();

    let bjs_after = bjs_after(&root, &nav.0[..], &from_to.root_all_lang_li())?;

    let nav = nav.json()?;
    let mut lang = Vec::with_capacity(lang_li.len());

    for i in &lang_li {
      let i = *i;
      let dir = root.join(LANG_CODE[i as usize]);
      let m: HashMap<String, String> = serde_yaml::from_slice(&ifs::r(dir.join("i18n.yml"))?)?;
      let mut bin = Vec::new();
      if !i18n_li.0.is_empty() {
        for i in &i18n_li.0 {
          if let Some(i) = m.get(i) {
            bin.extend(i.as_bytes());
          }
          bin.push(0);
        }
        bin.pop();
      }
      lang.push((i, bin));
    }
    let mnt = Mnt::load(
      &root,
      &conf.upload,
      ignore,
      // &nav.0,
      // &from_to,
      lang_li,
    )?;

    // _LANG = en name url
    let i = Self {
      lang,
      bjs_after,
      index_js,
      conf,
      css,
      root,
      i18n,
      htm,
      pug,
      nav,
      mnt,
    };
    Ok(i)
  }

  pub async fn build(&self, conf_name: &str) -> Result<VerFs> {
    let root = &self.root;
    let outdir = root.join("out").join(conf_name);
    let outv = outdir.join(V);
    let mut vfs = VerFs::load(
      root,
      outv,
      root.join(".i18n").join(V).join(conf_name).join("v.hash"),
    )?;

    if vfs.outver.exists() {
      std::fs::remove_dir_all(&vfs.outver)?;
    }

    let conf: HtmConf = yconf::load(&self.htm.join(format!("{}.yml", conf_name)))?;
    // let upload = upload(&conf)?;
    // let outdir = if let Some(ref outdir) = conf.outdir {
    // outdir
    // } else {
    //   PUBLIC
    // };

    let outhtm = outdir.join("htm");
    if outhtm.exists() {
      for i in fs::read_dir(&outhtm).unwrap() {
        if let Ok(i) = xerr::ok!(i) {
          let path = i.path();
          let rel = path.strip_prefix(&outhtm).unwrap().to_str().unwrap();
          if rel.starts_with(".") {
            continue;
          }

          if i.file_type().unwrap().is_dir() {
            xerr::log!(fs::remove_dir_all(path));
          } else {
            xerr::log!(fs::remove_file(path));
          }
        }
      }
      // xerr::log!(std::fs::remove_dir_all(&outhtm));
    }
    let public_dir = root.join(PUBLIC);
    let mut exist = HashSet::new();
    ifs::rsync(
      &public_dir,
      WalkDir::new(&public_dir).into_iter().filter_entry(|i| {
        exist.insert(
          i.path()
            .strip_prefix(&public_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
        );
        true
      }),
      // .filter_entry(|i| {
      //   let i = i
      //     .path()
      //     .strip_prefix(&public_dir)
      //     .unwrap()
      //     .to_str()
      //     .unwrap();
      //   // if i.starts_with('.') {
      //   //   return false;
      //   // }
      //   // i != "S.js"
      // }),
      &outhtm,
    )?;

    worker(root, &conf, &outhtm)?;

    let js_ver = self.js(&mut vfs, conf_name, &conf).await?;
    let prefix_index_ver = self.mnt.build(&mut vfs, &self.bjs_after.lang_path_bin)?;

    if vfs.has_new() {
      let v = format!("{js_ver}>{prefix_index_ver}");
      for i in [vfs.outver.join(".v"), vfs.out.join(".v")] {
        ifs::wbin(i, &v)?;
      }
    }
    Ok(vfs)
  }
}
