#[derive(Default, Debug)]
pub struct Restore {
  pub pos_li: Vec<(usize, String)>,
}

impl Restore {
  pub fn push(&mut self, pos: usize, str: String) {
    self.pos_li.push((pos, str));
  }

  pub fn load<S, I>(&self, li: I) -> String
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    let mut result = String::new();
    let mut pos_iter = self.pos_li.iter().peekable();

    for (pos, item) in li.into_iter().enumerate() {
      while let Some((p, s)) = pos_iter.peek() {
        if *p == pos {
          result.push_str(s);
          pos_iter.next();
        } else {
          break;
        }
      }

      result.push_str(item.as_ref());
    }

    for (_, s) in pos_iter {
      result.push_str(s);
    }

    result
  }
}
