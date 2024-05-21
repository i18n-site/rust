use std::path::Path;

use aok::{Null, OK};
use ft::FromTo;
use globset::GlobSet;
use i18::{build_ignore, EMPTY};
use ifs::unix_path;
use lang::{Lang, LANG_CODE, LANG_NAME};
use prost::Message;
use walkdir::WalkDir;

use crate::{api, css_js, foot_pug, js, nav_i18n, upload, Conf, Upload, VDir, S3};

pub async fn _gen(
  root: &Path,
  mut conf: Conf,
  mut vdir: VDir,
  ignore: GlobSet,
  all_lang: &[Lang],
  root_all_lang_li: &[Lang],
  mut upload: impl Upload,
) -> Null {
  let mut add = |lang, rel: String| {
    vdir.add(&rel);
    upload.add(lang, &rel);
  };

  let mut nav_i18n_li = Vec::with_capacity(conf.nav.len());
  let mut nav_li = conf
    .nav
    .iter_mut()
    .map(|i| {
      macro_rules! nav{
          ($($k:ident),+)=>{
            api::Nav {
              $(
                $k: i.get(stringify!($k)).unwrap_or(&String::new()).into(),
              )+
              arg:i.get("arg").cloned(),
            }
          }
        }
      let nav = nav!(i18n, url, page, menu);
      if !nav.i18n.is_empty() {
        nav_i18n_li.push(nav.i18n.clone());
      }
      nav
    })
    .collect();

  js(root, root_all_lang_li, &conf, &mut nav_li, &mut add);

  let foot = foot_pug(root, &mut nav_i18n_li);

  let public = root.join("public");
  let public_lang = public.join("lang");
  let mut lang_li = Vec::with_capacity(all_lang.len());

  let mut lang_bin_li = Vec::with_capacity(all_lang.len());

  // 遍历所有目录, 根据vdir划分到不同的vtab
  for lang in all_lang {
    let lang_usize = *lang as usize;
    let en = LANG_CODE[lang_usize];

    let nav_lang = nav_i18n(root, en);

    let url = format!("lang/{en}");
    let mut bin = vec![];

    for i in &nav_i18n_li {
      if !bin.is_empty() {
        bin.push(0);
      }
      bin.extend(nav_lang.get(i).unwrap_or(&EMPTY).as_bytes());
    }

    ifs::wbin(public_lang.join(en), &bin)?;

    lang_bin_li.push(bin);

    lang_li.push(api::Lang {
      name: LANG_NAME[lang_usize].into(),
      en: en.into(),
      url,
    });

    let dir = root.join(en);
    for entry in WalkDir::new(&dir).into_iter().filter_entry(dot_hide::not) {
      if let Ok(entry) = xerr::ok!(entry) {
        if entry.file_type().is_file() {
          if let Ok(rel) = xerr::ok!(entry.path().strip_prefix(&dir)) {
            let rel = unix_path(rel.as_os_str().to_str().unwrap());
            if ignore.is_match(format!("/{rel}")) {
              continue;
            }
            if rel == "i18n.yml" {
              continue;
            }
            let fp = entry.path();
            if let Some(ext) = fp.extension() {
              if let Some(ext) = ext.to_str() {
                if conf.upload.ext.contains(&ext.to_owned()) {
                  add(*lang, rel);
                  // vdir.add(&rel);
                  // upload.add(*lang, &rel);
                }
              }
            }
          }
        }
      }
    }
  }

  let vli = vdir.build(root)?;

  let len = vli.len();
  let mut vtab_li = Vec::with_capacity(len);
  let mut vlang_li = Vec::with_capacity(len);

  for (vtab, vlang) in vli {
    vtab_li.push(vtab);
    vlang_li.push(vlang);
  }

  let cj = css_js(root, &foot);
  let site = api::Site {
    id: conf.id,
    vtab_li,
    nav_li,
    lang_li,
    css: cj.css,
    js: cj.js,
  };

  let bin = site.encode_to_vec();
  ifs::wbin(public.join(".v"), bin)?;

  upload.upload(site, root, vlang_li, lang_bin_li).await?;
  OK
}

pub async fn gen(root: &Path, conf: Conf, vdir: VDir, upload_s3: bool) -> Null {
  let i18n = &conf.i18n;
  let ignore = build_ignore(&conf.ignore);

  i18::run(root, i18n, &ignore, i18::token()).await?;

  let from_to: FromTo = (&i18n.fromTo).into();
  let all_lang = &from_to.all_lang_li()[..];
  let root_all_lang_li = &from_to.root_all_lang_li()[..];

  macro_rules! gen {
    ($upload:expr) => {
      let upload = $upload;
      _gen(root, conf, vdir, ignore, all_lang, root_all_lang_li, upload).await?;
    };
  }

  if upload_s3 {
    gen!(S3::new(&conf, all_lang));
  } else {
    gen!(upload::No);
  };

  OK
}
