mod s3;
use std::{
  fs::File,
  io::{self, BufRead, BufReader, Write},
  path::{Path, PathBuf},
};

use aok::{Null, Result, OK};
use flate2::{write::GzEncoder, Compression};
use futures::{stream, stream::StreamExt};
use globset::GlobSet;
use gxhash::{HashMap, HashSet};
use i18::DOT_I18N;
use ifs::unix_path;
use lang::{Lang, LANG_CODE};
use s3::S3;
use walkdir::WalkDir;
use ytree::sitemap::{md_url, Sitemap};

pub trait Seo {
  fn init(root: &Path, name: &str, host: &str) -> Result<Self>
  where
    Self: Sized;

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null;
}

pub struct Fs {
  pub out: PathBuf,
}

impl Seo for Fs {
  fn init(root: &Path, name: &str, _host: &str) -> Result<Self> {
    let out = root.join("out").join(name).join("htm");
    Ok(Self { out })
  }

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
    ifs::wbin(self.out.join(rel.as_ref()), bin.as_ref())?;
    OK
  }
}

fn gz(data: impl AsRef<[u8]>) -> Result<Vec<u8>, io::Error> {
  let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
  encoder.write_all(data.as_ref())?;
  encoder.finish()
}

pub fn md2htm(fp: &Path) -> Result<Option<String>> {
  let md = std::fs::read_to_string(fp)?;
  {
    use std::io::Cursor;
    let c = Cursor::new(&md);
    let mut iter = c.lines();
    let mut n = 0;

    #[allow(clippy::never_loop)]
    'out: loop {
      while let Some(Ok(i)) = iter.next() {
        if !i.trim().is_empty() {
          n += 1;
          if n > 1 {
            break 'out;
          }
        }
      }
      return Ok(None);
    }
  }

  let mut opt = markdown::Options::gfm();
  opt.compile.allow_dangerous_html = true;
  opt.compile.allow_dangerous_protocol = true;
  if let Ok(htm) = xerr::ok!(markdown::to_html_with_options(&md, &opt)) {
    let htm = htm.replace(">\n<", "><");
    let htm = htm.trim_end();
    return Ok(Some(htm.into()));
  };
  Ok(None)
}

pub type LangRelHtm = Vec<(Lang, String, String)>;

pub const TOC: &str = "TOC";

async fn scan(
  root: &Path,
  lang_li: &[Lang],
  changed: &HashSet<String>,
  exist: &mut Sitemap,
  ignore: &GlobSet,
) -> Result<Option<LangRelHtm>> {
  let (to_insert, to_remove) = {
    let exist = &exist;
    let mut to_insert = vec![];
    let mut toc_dir = vec![];

    let mut to_remove = (*exist).clone();

    for lang in lang_li {
      let lang = *lang;
      let lang_en = LANG_CODE[lang as usize];
      let dir = root.join(lang_en);
      for entry in WalkDir::new(&dir).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          let full_path = entry.path();
          if let Ok(path) = full_path.strip_prefix(&dir)
            && let Some(path_rel) = path.to_str()
          {
            let path_rel = unix_path(path_rel);
            if let Ok(meta) = entry.metadata() {
              let file_type = meta.file_type();
              if file_type.is_dir() {
                let toc = full_path.join(TOC);
                if toc.exists() {
                  toc_dir.push(path_rel.to_owned());

                  let mut iter = rtoc::r(toc)?.into_iter().peekable();
                  while let Some(name) = iter.next() {
                    let rel = format!("{path_rel}/{name}");
                    for lang in lang_li {
                      let lang = *lang;
                      let lang_en = LANG_CODE[lang as usize];
                      let lang_rel = format!("{lang_en}/{rel}");
                      let fp = root.join(&lang_rel);
                      if fp.exists() {
                        if to_remove.remove(lang, &rel) && !changed.contains(&lang_rel) {
                          continue;
                        }
                        if let Some(htm) = md2htm(&root.join(fp))? {
                          to_insert.push((lang, rel.clone(), htm));
                        } else if name == "README.md" {
                          if let Some(next) = iter.peek() {
                            let url = format!("{lang_en}/{path_rel}/{next}");
                            let url = ytree::sitemap::md_url(&url);
                            let htm =
                              format!(r#"<meta http-equiv=refresh content="0;url=/{url}">"#);
                            to_insert.push((lang, rel.clone(), htm));
                          }
                        }
                      }
                    }
                  }
                }
              } else if file_type.is_file() {
                for i in &toc_dir {
                  if path_rel.starts_with(i) && path_rel[i.len()..].chars().next() == Some('/') {
                    continue;
                  }
                }
                if let Some(ext) = path.extension() {
                  if ext == "md" {
                    let fp = format!("{lang_en}/{path_rel}");
                    if ignore.is_match(format!("/{fp}")) {
                      continue;
                    }
                    if to_remove.remove(lang, &path_rel) && !changed.contains(&fp) {
                      continue;
                    }
                    if let Some(htm) = md2htm(&root.join(fp))? {
                      to_insert.push((lang, path_rel, htm));
                    }
                  }
                }
              }
            }
          };
        }
      }
    }
    (to_insert, to_remove.set())
  };

  if to_remove.is_empty() && to_insert.is_empty() {
    return Ok(None);
  }

  for (lang, rel) in to_remove {
    exist.remove(lang, rel);
  }

  for (lang, rel, _) in &to_insert {
    exist.insert(*lang, rel);
  }

  Ok(Some(to_insert))
}

pub async fn put(
  host: &str,
  upload: impl Seo,
  to_insert: LangRelHtm,
  exist: Sitemap,
) -> Result<String> {
  let upload = &upload;
  {
    let to_insert_len = to_insert.len();
    let mut iter = stream::iter(
      to_insert.into_iter().filter_map(|(lang, rel, htm)|{
          exist.rel_lang_set.get(&rel).map(|t| (t, lang, rel, htm))
      }).map(|(t, lang, rel, htm)| {
          let url = md_url(&rel).to_owned();
          let (url, url_htm) = if url.is_empty() {
            (
              "".into(),
              ".htm".into()
            )
          }else{
            let url = format!("/{url}");
            let url_htm = format!("{url}.htm");
            (
              url,
              url_htm
            )
          };

/*
https://google.github.io/styleguide/htmlcssguide.html#Optional_Tags
省略可选标签(html head body)。 HTML5 规范定义了哪些标签可以省略。
*/
          let htm = if htm.starts_with("<meta http-equiv=refresh") { 
            htm
          }else{
            format!(
              r#"<!doctypehtml><meta charset=UTF-8><link rel="alternate" href="https://{host}{url}" hreflang=x-default><link rel=canonical href="https://{host}{url}">{}<script src=//registry.npmmirror.com/18x/latest/files/seo.js></script>{htm}"#,
              t.lang_set
              .iter()
              .map(|lang| {
                let lang = LANG_CODE[lang as usize];
                format!(r#"<link rel=alternate hreflang={lang} href="https://{host}/{lang}{url_htm}">"#)
              })
              .collect::<Vec<_>>()
              .join("")
            )
          };

          let url = format!("{}{}", LANG_CODE[lang as usize], url_htm);
          async move {
            (
              upload.put(&url, htm).await,
              url
            )
          }
      })
    ).buffer_unordered(8);

    let mut bar = pbar::pbar(to_insert_len as u64);

    while let Some((r, url)) = iter.next().await {
      bar.inc(1);
      bar.set_message(format!("SEO ⬆ {url}"));
      r?;
    }
    bar.finish_and_clear();
  }

  let li = {
    let mut iter = stream::iter(exist.gen(host).into_iter().enumerate().map(
      |(pos, xml)| async move {
        let fp = format!("{}.xml.gz", pos);
        upload.put(&fp, gz(xml)?).await?;
        Ok::<String, aok::Error>(fp)
      },
    ))
    .buffer_unordered(8);

    let mut li = vec![];
    while let Some(r) = iter.next().await {
      if let Ok(r) = xerr::ok!(r) {
        li.push(r);
      }
    }
    li
  };

  let tsutc = tsfmt::utc(exist.now);
  let xml = format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</sitemapindex>"#,
    li.into_iter()
      .map(|fp| format!(
        "<sitemap><loc>https://{host}/{fp}</loc><lastmod>{tsutc}</lastmod></sitemap>"
      ))
      .collect::<Vec<_>>()
      .join("")
  );

  upload.put("sitemap.xml", xml).await?;
  /*
  <?xml version="1.0" encoding="UTF-8"?>
  <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
  <loc>https://example.com/sitemaps/sitemap_part_aa.gz</loc>
  </sitemap>
  <sitemap>
  <loc>https://example.com/sitemaps/sitemap_part_ab.gz</loc>
  </sitemap>
  </sitemapindex>
  */

  Ok(exist.dumps())
}

pub async fn gen<Upload: Seo>(
  host: &str,
  action: &str,
  root: &Path,
  name: &str,
  lang_li: &[Lang],
  ignore: &GlobSet,
  changed: &HashSet<String>,
) -> Null {
  let seo_fp = root.join(DOT_I18N).join("seo").join(host).join(action);
  let exist = if seo_fp.exists() {
    let reader = BufReader::new(File::open(&seo_fp)?);
    ytree::sitemap::loads(reader.lines().map_while(Result::ok))
  } else {
    Default::default()
  };
  let mut exist = exist.rel_lang_set(root)?;
  if let Ok(Some(to_insert)) = xerr::ok!(scan(root, lang_li, changed, &mut exist, ignore,).await) {
    let m = Upload::init(root, name, host)?;
    let yml = put(host, m, to_insert, exist).await?;
    ifs::wbin(seo_fp, yml)?;
  }
  OK
}

pub async fn seo(
  conf: &HashMap<String, String>,
  root: &Path,
  name: &str,
  lang_li: Vec<Lang>,
  ignore: &GlobSet,
  changed: &HashSet<String>,
) -> Null {
  for (host, action_li) in conf {
    for action in action_li.split_whitespace() {
      macro_rules! gen {
        ($seo:ty) => {
          gen::<$seo>(host, action, root, name, &lang_li, ignore, changed).await
        };
      }
      let r = match action {
        "fs" => gen!(Fs),
        "s3" => gen!(S3),
        _ => {
          eprintln!("unknown action {action}");
          continue;
        }
      };
      if let Err(e) = r {
        eprintln!("❌ seo {name} {host} {action} error : {e}");
      }
    }
  }
  OK
}
