use i18n_js::HtmConf;
use ytree::sitemap::{LangTree, Sitemap};

mod scan;
use scan::scan;
mod md_htm;
pub use md_htm::MdHtm;
mod rss;
use rss::Rss;

pub const README_MD: &str = "README.md";

use std::{
  fs::File,
  io::{self, BufRead, BufReader, Write},
  path::Path,
};

use aok::{Null, Result, OK};
use ckv::Ckv;
use flate2::{write::GzEncoder, Compression};
use futures::{stream, stream::StreamExt};
use globset::GlobSet;
use gxhash::{HashMap, HashSet};
use i18::DOT_I18N;
use lang::{Lang, LANG_CODE};
use ytree::sitemap::md_url;

fn gz(data: impl AsRef<[u8]>) -> Result<Vec<u8>, io::Error> {
  let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
  encoder.write_all(data.as_ref())?;
  encoder.finish()
}

pub type LangRelTitleHtm = Vec<(Lang, String, String, String)>;

pub async fn put(
  upload: &impl Ckv,
  to_insert: LangRelTitleHtm,
  css: &str,
  sitemap: &mut Sitemap,
  rss: &mut Rss,
) -> Null {
  let upload = &upload;
  let host = &rss.host;
  {
    let to_insert_len = to_insert.len() + rss.li.len();
    let mut bar = pbar::pbar(to_insert_len as u64);
    macro_rules! wait {
      ($gen:expr) => {{
        let mut iter = stream::iter($gen).buffer_unordered(64);
        while let Some((r, url)) = iter.next().await {
          bar.inc(1);
          bar.set_message(format!("SEO ⬆ {url}"));
          r?;
        }
        drop(iter);
      }};
    }

    wait!(
      to_insert.into_iter().filter_map(|(lang, rel, title, htm)|{
        let htm = if title.is_empty() {
          htm
        }else{
         format!("<title>{title}</title>{htm}")
        };
        sitemap.rel_lang_set.get(&rel).map(|t| (t, lang, rel, htm))
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
            r#"<!doctypehtml><meta charset=UTF-8><link rel=stylesheet href=//registry.npmmirror.com/{css}/latest/files/_.css><script src=//registry.npmmirror.com/18x/latest/files/seo.js></script><link rel=alternate href="https://{host}{url}" hreflang=x-default><link rel=stylesheet href=//registry.npmmirror.com/18x/latest/files/seo.css>{}{htm}"#,
            t.lang_set
            .iter()
            .map(|lang| {
              let lang = LANG_CODE[lang as usize];
              format!(r#"<link rel=alternate hreflang={lang} href="https://{host}/{lang}{url_htm}">"#)
            })
            .collect::<Vec<_>>()
            .join(""),
          );

          let url = format!("{}{}", LANG_CODE[lang as usize], url_htm);
          async move {
            (
              upload.put(&url, htm).await,
              url
            )
          }
      })
    );

    wait!(rss
      .gen()
      .into_iter()
      .map(|(url, htm)| async move { (upload.put(&url, htm).await, url) }));

    bar.finish_and_clear();
  }

  let li = {
    let mut iter = stream::iter(sitemap.gen(host).into_iter().enumerate().map(
      |(pos, xml)| async move {
        let fp = format!("sitemap/{}.xml.gz", pos);
        upload.put(&fp, gz(xml)?).await?;
        Ok::<String, aok::Error>(fp)
      },
    ))
    .buffer_unordered(32);

    let mut li = vec![];
    while let Some(r) = iter.next().await {
      if let Ok(r) = xerr::ok!(r) {
        li.push(r);
      }
    }
    li
  };

  let tsutc = tsfmt::utc(sitemap.now);
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
  OK
}

pub fn load_lang_tree(seo_fp: &Path) -> Result<LangTree> {
  Ok(if seo_fp.exists() {
    let reader = BufReader::new(File::open(seo_fp)?);
    ytree::sitemap::loads(reader.lines().map_while(Result::ok))
  } else {
    Default::default()
  })
}

pub async fn gen(
  upload: &impl Ckv,
  host: &str,
  kind: &str,
  root: &Path,
  lang_li: &[Lang],
  ignore: &GlobSet,
  changed: &HashSet<String>,
  foot: &HashMap<Lang, String>,
  css: &str,
) -> Null {
  let seo_dir = root.join(DOT_I18N).join("seo").join(host).join(kind);

  let sitemap_fp = seo_dir.join("sitemap");
  let rss_fp = seo_dir.join("rss");

  let mut sitemap = load_lang_tree(&sitemap_fp)?.sitemap(root)?;

  let mut rss = Rss::new(root, host, load_lang_tree(&rss_fp)?.lang_rel_ts());

  // let m = U::init(root, name, host)?;
  if let Ok(Some(to_insert)) = xerr::ok!(
    scan(
      host,
      root,
      lang_li,
      changed,
      ignore,
      foot,
      &mut sitemap,
      &mut rss
    )
    .await
  ) {
    put(upload, to_insert, css, &mut sitemap, &mut rss).await?;
    ifs::wbin(sitemap_fp, sitemap.dumps())?;
    ifs::wbin(rss_fp, rss.dumps())?;
  }
  OK
}

pub async fn seo(
  kind: &str,
  upload: &impl Ckv,
  conf: &HtmConf,
  root: &Path,
  lang_li: &[Lang],
  ignore: &GlobSet,
  changed: &HashSet<String>,
  foot: &HashMap<Lang, String>,
) -> Null {
  if !conf.seo {
    return OK;
  }
  let css = &conf.x;
  let host = &conf.host;
  gen(
    upload, host, kind, root, lang_li, ignore, changed, foot, css,
  )
  .await
}
