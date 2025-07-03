#[derive(Debug, Clone, Copy)]
pub struct DocChunk {
  pub limit: usize,
}

impl DocChunk {
  pub fn new(limit: usize) -> Self {
    Self { limit }
  }

  pub fn parse(&self, txt: impl AsRef<str>) -> Vec<String> {
    let limit = self.limit;

    let mut result = vec![];

    let mut count = 0;

    let mut tmp = String::new();

    for line in txt.as_ref().lines() {
      let line = line.trim_end();
      if line.trim_start().is_empty() {
        continue;
      }
      let n = count + line.len() + 1;

      let tmp_not_empty = !tmp.is_empty();
      if n > limit {
        count = 0;
        if tmp_not_empty {
          result.push(tmp.to_owned());
        }
        let len = line.len();
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
      } else {
        count = n;
        if tmp_not_empty {
          tmp.push('\n');
        }
        tmp.push_str(line);
      }
    }

    result
  }
}
