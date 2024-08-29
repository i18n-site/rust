use std::{
  io::{BufRead, Cursor},
  path::Path,
};

use md_title::md_title;

pub fn wrap(htm: &str) -> String {
  if !htm.is_empty() {
    return format!("<article>{htm}</article>");
  }

  htm.into()
}

pub fn only_title(md: &str) -> bool {
  let c = Cursor::new(&md);
  let mut iter = c.lines();
  let mut n = 0;

  while let Some(Ok(i)) = iter.next() {
    if !i.trim().is_empty() {
      n += 1;
      if n > 1 {
        return false;
      }
    }
  }

  true
}

pub struct MdHtm {
  pub _title: Option<String>,
  pub md: String,
  pub only_title: bool,
}

impl MdHtm {
  pub fn load(fp: impl AsRef<Path>) -> std::io::Result<Self> {
    let md = std::fs::read_to_string(fp)?;
    Ok(Self {
      _title: None,
      only_title: only_title(&md),
      md,
    })
  }

  pub fn title(&mut self) -> &str {
    if self._title.is_none() {
      let title = md_title(&self.md);

      self._title = Some(if title.is_empty() {
        title
      } else {
        htmlize::escape_text(title).into()
      });
    }
    self._title.as_ref().unwrap()
  }

  pub fn htm(&mut self) -> Option<String> {
    if self.only_title {
      None
    } else {
      let title = self.title();
      let mut htm = if title.is_empty() {
        "".into()
      } else {
        format!(r#"<title>{title}</title>"#)
      };

      let mut opt = markdown::Options::gfm();
      let compile = &mut opt.compile;
      compile.allow_dangerous_html = true;
      compile.allow_dangerous_protocol = true;
      compile.gfm_tagfilter = false;
      let md = &self.md;
      htm += &if let Ok(h) = xerr::ok!(markdown::to_html_with_options(md, &opt)) {
        let h = h.replace(">\n<", "><");
        wrap(h.trim_end())
      } else {
        wrap(&format!("<pre>{md}</pre>"))
      };
      Some(htm)
    }
  }
}
