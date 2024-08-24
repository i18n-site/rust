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
use walkdir::WalkDir;
use ytree::sitemap::{md_url, Sitemap};

fn gz(data: impl AsRef<[u8]>) -> Result<Vec<u8>, io::Error> {
  let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
  encoder.write_all(data.as_ref())?;
  encoder.finish()
}

pub trait Seo {
  fn init(root: &Path, name: &str, host: String) -> Result<Self>
  where
    Self: Sized;

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null;
}

pub struct Fs {
  pub out: PathBuf,
}

impl Seo for Fs {
  fn init(root: &Path, name: &str, _host: String) -> Result<Self> {
    let out = root.join("out").join(name).join("htm");
    Ok(Self { out: out.into() })
  }

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
    ifs::wbin(&self.out.join(rel.as_ref()), bin.as_ref())?;
    OK
  }
}

pub fn md2htm(fp: &Path) -> Result<Option<String>> {
  let md = std::fs::read_to_string(fp)?;
  {
    use std::io::Cursor;
    let c = Cursor::new(&md);
    let mut iter = c.lines();
    let mut n = 0;
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

async fn upload(
  upload: &impl Seo,
  host: &str,
  root: &Path,
  lang_li: &[Lang],
  changed: &HashSet<String>,
  mut exist: Sitemap,
  ignore: &GlobSet,
) -> Result<Option<String>> {
  let (to_insert, mut to_remove) = {
    let exist = &exist;
    let mut to_insert = vec![];
    let mut to_remove = vec![];
    for lang in lang_li {
      let lang = *lang;
      let lang_en = LANG_CODE[lang as usize];
      let dir = root.join(lang_en);
      for entry in WalkDir::new(&dir).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          let path = entry.path();
          if let Some(ext) = path.extension() {
            if ext == "md" {
              if let Ok(meta) = entry.metadata() {
                if meta.file_type().is_file() {
                  if let Ok(path) = path.strip_prefix(&dir) {
                    if let Some(rel) = path.to_str() {
                      let rel = unix_path(rel);
                      let fp = format!("{lang_en}/{rel}");
                      if ignore.is_match(format!("/{fp}")) {
                        continue;
                      }
                      if !changed.contains(&fp) {
                        if exist.contains(lang, &rel) {
                          continue;
                        }
                      }
                      if let Some(htm) = md2htm(&root.join(fp))? {
                        to_insert.push((lang, rel, htm));
                      } else if exist.contains(lang, &rel) {
                        to_remove.push((lang, rel));
                      }
                    }
                  }
                }
              }
            }
          };
        }
      }
    }
    (to_insert, to_remove)
  };

  if to_remove.is_empty() && to_insert.is_empty() {
    return Ok(None);
  }

  while let Some((lang, rel)) = to_remove.pop() {
    exist.remove(lang, rel);
  }

  for (lang, rel, _) in &to_insert {
    exist.insert(*lang, rel);
  }

  {
    let mut iter = stream::iter(
      to_insert.into_iter().filter_map(|(lang, rel, htm)|{
          if let Some(t) = exist.rel_lang_set.get(&rel) {
            Some((t, lang, rel, htm))
          } else {
            None
          }
      }).map(|(t, lang, rel, htm)| {
          let url = md_url(&rel).to_owned();
          let (url, url_htm) = if url.is_empty() {
            (
              "".into(),
              ".htm".into()
            )
          }else{
            (
              format!("/{url}"),
              format!("/{url}.htm")
            )
          };

          let htm = format!(
            r#"<!doctypehtml><head><meta charset=UTF-8><link rel="alternate" href="https://{host}{url}" hreflang=x-default><link rel=canonical href="https://{host}{url}">{}<script src=//registry.npmmirror.com/18x/latest/files/seo.js></script></head><body>{htm}</body>"#,
            t.lang_set
              .iter()
              .map(|lang| {
                let lang = LANG_CODE[lang as usize];
                format!(r#"<link rel=alternate hreflang={lang} href="https://{host}/{lang}{url_htm}">"#)
              })
              .collect::<Vec<_>>()
              .join("")
          );
          upload.put(format!("{}{}", LANG_CODE[lang as usize], url_htm), htm)
      })
    ).buffer_unordered(6);
    while let Some(r) = iter.next().await {
      xerr::log!(r);
    }
  }

  let li = {
    let mut iter = stream::iter(exist.gen(host).into_iter().enumerate().map(
      |(pos, xml)| async move {
        let fp = format!("sitemap{}.gz", pos);
        upload.put(&fp, gz(xml)?).await?;
        Ok::<String, aok::Error>(fp)
      },
    ))
    .buffer_unordered(6);

    let mut li = vec![];
    while let Some(r) = iter.next().await {
      if let Ok(r) = xerr::ok!(r) {
        li.push(r);
      }
    }
    li
  };

  let xml = format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</sitemapindex>"#,
    li.into_iter()
      .map(|fp| format!(
        "<sitemap><loc>https://{host}/{fp}</loc><lastmod>2023-01-23</lastmod></sitemap>"
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

  Ok(Some(exist.dumps()))
}

pub async fn seo(
  root: &Path,
  name: &str,
  conf: &HashMap<String, String>,
  lang_li: Vec<Lang>,
  ignore: &GlobSet,
  changed: &HashSet<String>,
) -> Null {
  for (host, action) in conf {
    for action in action.split_whitespace() {
      let m = {
        let host = host.clone();
        if action == "fs" {
          Fs::init(root, name, host)
        } else {
          eprintln!("seo {name} {host} {action} not support");
          continue;
        }
      };

      match m {
        Err(e) => {
          eprintln!("seo {name} {host} {action} {e}");
          continue;
        }
        Ok(m) => {
          let seo_fp = root.join(DOT_I18N).join("seo").join(host).join(action);
          let exist = if seo_fp.exists() {
            let reader = BufReader::new(File::open(&seo_fp)?);
            ytree::sitemap::loads(reader.lines().filter_map(|i| i.ok()))
          } else {
            Default::default()
          };
          if let Ok(Some(yml)) = xerr::ok!(
            upload(
              &m,
              host,
              root,
              &lang_li,
              changed,
              exist.rel_lang_set(root)?,
              ignore,
            )
            .await
          ) {
            ifs::wbin(seo_fp, yml)?;
          }
        }
      }
    }
  }

  OK
}
