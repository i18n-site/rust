use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use mdli::{Kind, MdLi};

#[derive(Clone)]
pub struct VarUrl {
  pub ac: Option<CharwiseDoubleArrayAhoCorasick<usize>>,
}

impl VarUrl {
  pub fn new<I, S: AsRef<str>>(prefix_li: I) -> Self
  where
    I: IntoIterator<Item = S>,
  {
    match CharwiseDoubleArrayAhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostLongest)
      .build(prefix_li.into_iter().filter(|i| !i.as_ref().is_empty()))
    {
      Ok(ac) => VarUrl { ac: Some(ac) },
      Err(err) => {
        match err {
          daachorse::errors::DaachorseError::InvalidArgument(_) => {}
          _ => {
            tracing::error!("{}", err);
          }
        };
        VarUrl { ac: None }
      }
    }
  }

  fn find_end<'a>(&self, before: &'a str, after: &'a str) -> Option<(usize, &'a str)> {
    if (before.ends_with("src=\"") || before.ends_with("href=\""))
      && let Some(url_end) = after.find('"')
    {
      return Some((url_end, &after[..url_end]));
    }
    None
  }

  // from_lang: &str, to_lang: &str
  pub fn replace(
    &self,
    mdli: &mut MdLi,
    from_to: impl Fn(usize) -> Option<(&'static str, &'static str)>,
  ) {
    if let Some(ac) = &self.ac {
      // let from_lang = format!("/{from_lang}/");
      // let to_lang = format!("/{to_lang}/");

      // 就地修改每个 Md 元素
      for i in 0..mdli.li.len() {
        if ![Kind::Img, Kind::Url, Kind::HtmOpen].contains(&mdli.li[i].kind) {
          continue;
        }

        let md = &mdli.li[i].str;
        let mut pre_pos = 0;
        let mut new_str = String::new();
        let mut last_end = 0; // 记录上一次匹配的结束位置

        for m in ac.leftmost_find_iter(md) {
          if let Some((from_lang, to_lang)) = from_to(m.value()) {
            let start = m.start();
            let end = m.end() - 1;
            let val = &md[start..end];

            if start == 0 {
              new_str += val;
              new_str += &md[end..].replace(from_lang, to_lang);
              pre_pos = md.len();
              continue;
            }

            // 如果当前开始位置小于上一次的结束位置，跳过这次匹配
            if start < last_end {
              continue;
            }

            let before = &md[..start];
            let after = &md[end..];

            if let Some((url_end, url_part)) = self.find_end(before, after)
              && url_part.contains(from_lang)
            {
              let new_url = format!("{}{}", val, url_part.replace(from_lang, to_lang));

              // 添加前面的文本和新URL
              new_str.push_str(&md[pre_pos..start]);
              new_str.push_str(&new_url);
              pre_pos = end + url_end;
              last_end = pre_pos; // 更新上一次的结束位置
              continue;
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
          mdli.li[i].str = new_str;
        }
      }
    }
  }
}
