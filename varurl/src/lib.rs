#![feature(let_chains)]

use daachorse::{
  errors::Result, CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind,
};
use mdli::{Kind, Md};

pub struct MdLi<'a>(pub Vec<Md<'a>>);

impl<'a> MdLi<'a> {
  pub fn join(&self) -> String {
    self.0.iter().map(|md| md.str).collect()
  }
}

pub struct VarUrl {
  pub ac: CharwiseDoubleArrayAhoCorasick<usize>,
  pub from_lang: String,
}

impl VarUrl {
  pub fn new<I, S: AsRef<str>>(patterns: I, from_lang: &str) -> Result<Self>
  where
    I: IntoIterator<Item = S>,
  {
    let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostLongest)
      .build(patterns)?;
    Ok(VarUrl {
      ac,
      from_lang: format!("/{from_lang}/"),
    })
  }

  fn find_end<'a>(&self, before: &'a str, after: &'a str) -> Option<(usize, &'a str)> {
    if before.ends_with("](") {
      if let Some(url_end) = after.find(')') {
        return Some((url_end, &after[..url_end]));
      }
    }

    if before.ends_with("src=\"") || before.ends_with("href=\"") {
      if let Some(url_end) = after.find('"') {
        return Some((url_end, &after[..url_end]));
      }
    }

    None
  }

  pub fn replace<'a>(&self, mdli: &mut MdLi<'a>, to_lang: &str) -> Result<()> {
    let to_lang = format!("/{to_lang}/");
    let mut new_vec = Vec::new();

    for i in &mdli.0 {
      if i.kind != Kind::Txt {
        new_vec.push(Md {
          kind: i.kind,
          str: i.str,
        });
        continue;
      }
      let md = i.str;
      let mut pre_pos = 0;
      let md_len = md.len();

      dbg!("处理文本:", md);
      let mut current_vec = Vec::new();

      for m in self.ac.leftmost_find_iter(md) {
        let start = m.start();
        let end = m.end();
        let val = &md[start..end];

        dbg!("找到URL前缀:", val, "位置:", start, end);

        let before = &md[..start];
        let after = &md[end..];

        dbg!("前文:", before);
        dbg!("后文:", after);

        if let Some((url_end, url_part)) = self.find_end(before, after) {
          dbg!("URL部分:", url_part);

          let full_url = format!("{}{}", val, url_part);
          dbg!("完整URL:", &full_url);

          if full_url.contains(&self.from_lang) {
            let new_url = full_url.replace(&self.from_lang, &to_lang);
            dbg!("新URL:", &new_url);

            if pre_pos < start {
              current_vec.push(Md {
                kind: Kind::Txt,
                str: &md[pre_pos..start],
              });
            }
            current_vec.push(Md {
              kind: Kind::Txt,
              str: &new_url,
            });
            pre_pos = end + url_end;
            continue;
          }
        }

        if pre_pos < end {
          current_vec.push(Md {
            kind: Kind::Txt,
            str: &md[pre_pos..end],
          });
        }
        pre_pos = end;
      }

      if pre_pos < md_len {
        current_vec.push(Md {
          kind: Kind::Txt,
          str: &md[pre_pos..],
        });
      }

      new_vec.extend(current_vec);
    }

    mdli.0 = new_vec;
    Ok(())
  }
}
