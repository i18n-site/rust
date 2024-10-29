#![feature(let_chains)]

use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use mdli::{Kind, MdLi};

#[derive(Clone)]
pub struct VarUrl {
  pub ac: CharwiseDoubleArrayAhoCorasick<usize>,
}

impl VarUrl {
  pub fn new<I, S: AsRef<str>>(prefix_li: I) -> Self
  where
    I: IntoIterator<Item = S>,
  {
    match CharwiseDoubleArrayAhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostLongest)
      .build(prefix_li)
    {
      Ok(ac) => VarUrl { ac },
      Err(err) => {
        tracing::error!("{}", err);
        VarUrl {
          ac: CharwiseDoubleArrayAhoCorasick::new::<_, &str>([]).unwrap(),
        }
      }
    }
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

  // from_lang: &str, to_lang: &str
  pub fn replace(
    &self,
    mdli: &mut MdLi,
    from_to: impl Fn(usize) -> Option<(&'static str, &'static str)>,
  ) {
    // let from_lang = format!("/{from_lang}/");
    // let to_lang = format!("/{to_lang}/");

    // 就地修改每个 Md 元素
    for i in 0..mdli.0.len() {
      if mdli.0[i].kind != Kind::Txt {
        continue;
      }

      let md = &mdli.0[i].str;
      let mut pre_pos = 0;
      let mut new_str = String::new();
      let mut last_end = 0; // 记录上一次匹配的结束位置

      for m in self.ac.leftmost_find_iter(md) {
        if let Some((from_lang, to_lang)) = from_to(m.value()) {
          let start = m.start();
          let end = m.end();

          // 如果当前开始位置小于上一次的结束位置，跳过这次匹配
          if start < last_end {
            continue;
          }

          let val = &md[start..end];

          let before = &md[..start];
          let after = &md[end..];

          if let Some((url_end, url_part)) = self.find_end(before, after) {
            let full_url = format!("{}{}", val, url_part);

            if full_url.contains(from_lang) {
              let new_url = full_url.replace(from_lang, to_lang);

              // 添加前面的文本和新URL
              new_str.push_str(&md[pre_pos..start]);
              new_str.push_str(&new_url);
              pre_pos = end + url_end;
              last_end = pre_pos; // 更新上一次的结束位置
              continue;
            }
          }

          // 添加未匹配的部分
          new_str.push_str(&md[pre_pos..end]);
          pre_pos = end;
          last_end = end; // 更新上一次的结束位置
        }
      }

      // 添加剩余的文本
      if pre_pos < md.len() {
        new_str.push_str(&md[pre_pos..]);
      }

      // 只有当文本有变化时才替换
      if new_str != *md {
        mdli.0[i].str = new_str;
      }
    }
  }
}
