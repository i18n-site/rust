pub type ChunkLi = Vec<Vec<String>>;

pub fn txt_chunk<S: Into<String>>(li: impl IntoIterator<Item = S>, limit: usize) -> ChunkLi {
  let mut r = vec![];
  let mut t = vec![];
  let mut len = 0;
  for i in li {
    let i = i.into();
    let i_len = 1 + i.len();
    let next_len = len + i_len;
    if next_len > limit {
      r.push(t);
      if i_len < limit {
        t = vec![i];
        len = i_len;
      } else {
        let mut end = limit;
        while !i.is_char_boundary(end) && end > 0 {
          end -= 1;
        }
        if end > 0 {
          r.push(vec![i[..end].into()]);
        }
        t = vec![];
        len = 0;
      }
    } else {
      len = next_len;
      t.push(i);
    }
  }

  if !t.is_empty() {
    r.push(t);
  }

  r
}
