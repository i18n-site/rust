use std::path::Path;

use lang::{Lang, LANG_CODE};
use md_title::md_title_from_path;

const BODY: &str = "<body";

pub fn index_html(root: &Path, fp: &str, lang_li: &[Lang]) -> std::io::Result<String> {
  let html = ifs::rstr(fp)?;
  let mut li = html.split(BODY);
  if let Some(prefix) = li.next()
    && let Some(end) = li.remainder()
  {
    let mut html = prefix.to_owned();
    for lang in lang_li {
      let en = LANG_CODE[*lang as usize];
      let fp = root.join(LANG_CODE[*lang as usize]).join("README.md");
      if let Ok(title) = md_title_from_path(fp) {
        let link = format!(
          r#"<link rel="alternate" type="application/rss+xml" title="{title}" hreflang="{en}" href="/{en}.rss">"#
        );
        html += &link;
      }
    }

    html += BODY;
    html += end;
    return Ok(html);
  }

  Ok(html)
}
