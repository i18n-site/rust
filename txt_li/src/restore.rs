#[derive(Default, Debug)]
pub struct Restore {
  pub li: Vec<(usize, String)>,
}

impl Restore {
  pub fn push(&mut self, pos: usize, str: String) {
    self.li.push((pos, str));
  }

  pub fn load<S, I>(&self, iter: I) -> String
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    let mut result = String::new();
    let mut pos_iter = self.li.iter().peekable();

    for (pos, item) in iter.into_iter().enumerate() {
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
