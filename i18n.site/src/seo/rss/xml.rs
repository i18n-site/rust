use std::path::PathBuf;

use md_title::md_title;
use ytree::sitemap::md_url;

use super::super::{MdHtm, README_MD};

pub struct Xml<'a> {
  host: &'a str,
  root: PathBuf,
  title: String,
  li: Vec<String>,
  lang: &'a str,
  ts: String,
}

impl<'a> Xml<'a> {
  pub fn new(host: &'a str, root: PathBuf, lang: &'a str) -> Self {
    let readme = root.join(README_MD);
    let title = if readme.exists()
      && let Ok(md) = xerr::ok!(std::fs::read_to_string(&readme))
    {
      let title = md_title(&md);

      if title.is_empty() {
        host.into()
      } else {
        title
      }
    } else {
      host.into()
    };

    Self {
      host,
      root,
      lang,
      ts: Default::default(),
      title,
      li: Vec::new(),
    }
  }

  pub fn add_rel(&mut self, ts: u64, rel: &str) {
    let fp = self.root.join(rel);
    if let Ok(mut md) = xerr::ok!(MdHtm::load(fp)) {
      if let Some(htm) = md.htm() {
        self.add(ts, rel, md.title(), &htm);
      }
    }
  }

  pub fn add(&mut self, ts: u64, rel: &str, title: &str, htm: &str) {
    let ts = tsfmt::utc(ts);
    let url = md_url(rel);
    let item = format!(
      r#"<item><title>{title}</title><link>https://{host}/{lang}{url}.htm</link><pubDate>{ts}</pubDate><description><![CDATA[{htm}]]></description></item>"#,
      host = self.host,
      url = if url.starts_with("/") {
        url.into()
      } else {
        format!("/{url}")
      },
      lang = self.lang,
      title = title,
      ts = ts,
      htm = htm
    );
    if self.ts.is_empty() {
      self.ts = ts;
    }
    self.li.push(item);
  }

  pub fn gen(&self) -> String {
    let li = self.li.join("");
    format!(
      r#"<?xml version="1.0" encoding="UTF-8"?><rss version="2.0"><channel><title>{title}</title><link>https://{host}</link><language>{lang}</language><lastBuildDate>{ts}</lastBuildDate>{li}</channel></rss>"#,
      title = self.title,
      host = self.host,
      lang = self.lang,
      ts = self.ts,
      li = li,
    )
  }
}
