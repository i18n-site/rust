#[derive(Debug, Clone, Copy)]
pub struct DocChunk {
  pub limit: usize,
}

impl DocChunk {
  pub const fn new(limit: usize) -> Self {
    Self { limit }
  }

  pub fn parse(&self, txt: impl AsRef<str>) -> Vec<String> {
    let limit = self.limit;

    let mut result = vec![];

    let mut count = 0;

    let mut tmp = String::new();

    macro_rules! parse {
      ($line:expr) => {{
        let line = $line;
        let len = line.len();

        let line = line.trim_end();
        if line.trim_start().is_empty() {
          continue;
        }
        let n = count + len + 1;

        let tmp_not_empty = !tmp.is_empty();
        if n > limit {
          if tmp_not_empty {
            result.push(tmp.to_owned());
          }
          if len > limit {
            let mut end = limit;
            while !line.is_char_boundary(end) {
              end -= 1;
            }
            result.push(line[..end].into());

            tmp = String::new();
          } else {
            tmp = line.into();
          }
          count = tmp.len();
        } else {
          count = n;
          if tmp_not_empty {
            tmp.push('\n');
          }
          tmp.push_str(line);
        }
      }};
    }

    for line in txt.as_ref().lines() {
      if line.len() > limit {
        use unicode_segmentation::UnicodeSegmentation;
        let mut t = String::new();
        for i in line.unicode_sentences() {
          let i = i.trim();
          let len = i.len();
          if len > 0 {
            t.push_str(i);
            if i.len() > 16 && t.len() > 95 {
              parse!(&t);
              t = String::new();
            }
          }
        }
        if !t.is_empty() {
          parse!(&t);
        }
      } else {
        parse!(line);
      }
    }

    if !tmp.is_empty() {
      result.push(tmp);
    }
    result
  }
}
