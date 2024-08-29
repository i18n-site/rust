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
use lang::{Lang, LANG_CODE};
use ytree::sitemap::{md_url, Sitemap};

mod scan;
use scan::scan;
mod md_htm;
pub use md_htm::MdHtm;
mod s3;
use s3::S3;

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

pub type LangRelHtm = Vec<(Lang, String, String)>;

pub async fn put(
  host: &str,
  upload: impl Seo,
  to_insert: LangRelHtm,
  exist: Sitemap,
  foot: &HashMap<Lang, String>,
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
            let url = if url.starts_with("/") {
              url
            }else{
              format!("/{url}")
            };
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
          let htm = format!(
            r#"<!doctypehtml><meta charset=UTF-8><link rel=stylesheet href=//registry.npmmirror.com/18x/latest/files/seo.css><script src=//registry.npmmirror.com/18x/latest/files/seo.js></script><link rel=alternate href="https://{host}{url}" hreflang=x-default><link rel=canonical href="https://{host}{url}">{}{htm}{}"#,
            t.lang_set
            .iter()
            .map(|lang| {
              let lang = LANG_CODE[lang as usize];
              format!(r#"<link rel=alternate hreflang={lang} href="https://{host}/{lang}{url_htm}">"#)
            })
            .collect::<Vec<_>>()
            .join(""),
            foot.get(&lang).unwrap_or(&"".into()),
          );

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
        let fp = format!("sitemap/{}.xml.gz", pos);
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
        "<sitemap><loc>https://{host}/{fp}</loc><lastmod>{tsutc}+00:00</lastmod></sitemap>"
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
  kind: &str,
  root: &Path,
  name: &str,
  lang_li: &[Lang],
  ignore: &GlobSet,
  changed: &HashSet<String>,
  foot: &HashMap<Lang, String>,
) -> Null {
  let seo_fp = root.join(DOT_I18N).join("seo").join(host).join(kind);
  let exist = if seo_fp.exists() {
    let reader = BufReader::new(File::open(&seo_fp)?);
    ytree::sitemap::loads(reader.lines().map_while(Result::ok))
  } else {
    Default::default()
  };
  let mut exist = exist.rel_lang_set(root)?;
  if let Ok(Some(to_insert)) =
    xerr::ok!(scan(host, root, lang_li, changed, &mut exist, ignore).await)
  {
    let m = Upload::init(root, name, host)?;
    let yml = put(host, m, to_insert, exist, foot).await?;
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
  foot: &HashMap<Lang, String>,
) -> Null {
  for (host, kind_li) in conf {
    for kind in kind_li.split_whitespace() {
      macro_rules! gen {
        ($seo:ty) => {
          gen::<$seo>(host, kind, root, name, &lang_li, ignore, changed, foot).await
        };
      }
      let r = match kind {
        "fs" => gen!(Fs),
        "s3" => gen!(S3),
        _ => {
          eprintln!("unknown kind {kind}");
          continue;
        }
      };
      if let Err(e) = r {
        eprintln!("❌ seo {name} {host} {kind} error : {e}");
      }
    }
  }
  OK
}
