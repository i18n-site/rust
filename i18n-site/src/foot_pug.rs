use std::{io::Cursor, path::Path};

use aok::Result;
use ipug::parse;

pub fn _foot_pug(fp: &Path, nav_i18n_li: &mut Vec<String>) -> Result<String> {
  let pug = ifs::rtxt(fp)?;

  let extract = i18n_parse::extract(&pug);
  let replace = extract
    .str
    .into_iter()
    .map(|s| {
      let p = if let Some(p) = nav_i18n_li.iter().position(|i| i == s) {
        p
      } else {
        let p = nav_i18n_li.len();
        nav_i18n_li.push(s.into());
        p
      };
      format!("I[{p}]")
    })
    .collect::<Vec<_>>();

  let pug = i18n_parse::replace(&pug, &extract.range, &replace);
  let mut buffer = Vec::new(); // 创建一个Vec<u8>，之后可以转换为String
  {
    let mut writer = Cursor::new(&mut buffer); // 创建Cursor来包装buffer
    parse(pug)?.to_html(&mut writer)?;
  }

  Ok(String::from_utf8(buffer)?)
}

pub fn foot_pug(root: &Path, nav_i18n_li: &mut Vec<String>) -> String {
  let fp = root.join(".i18n/htm/foot.pug");
  if fp.exists() {
    match _foot_pug(&fp, nav_i18n_li) {
      Ok(s) => {
        return s;
      }
      Err(err) => {
        tracing::error!("{}:\n{}", fp.display(), err);
      }
    }
  }
  "".into()
}
