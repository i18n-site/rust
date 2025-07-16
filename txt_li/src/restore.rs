#[derive(Default, Debug)]
pub struct Restore {
  pub pos_li: Vec<(usize, String)>,
}

impl Restore {
  pub fn push(&mut self, pos: usize, str: String) {
    self.pos_li.push((pos, str));
  }
}

impl<S, I> std::ops::Shl<I> for Restore
where
  I: IntoIterator<Item = S>,
  S: AsRef<str>,
{
  type Output = String;

  fn shl(self, li: I) -> Self::Output {
    let mut result = String::new();
    let mut current_pos = 0;
    let mut pos_iter = self.pos_li.iter().peekable();

    for item in li.into_iter() {
      while let Some((p, s)) = pos_iter.peek() {
        if *p == current_pos {
          result.push_str(s);
          pos_iter.next();
        } else {
          break;
        }
      }

      result.push_str(item.as_ref());
      current_pos += 1;
    }

    for (_, s) in pos_iter {
      result.push_str(s);
    }

    result
  }
}
