use crate::TxtPos;

impl<'a> TxtPos<'a> {
  pub fn extend(&mut self, tp: TxtPos<'a>) {
    let offset = self.txt_li.len();
    if offset == 0 {
      self.pos_li = tp.pos_li;
    } else {
      self
        .pos_li
        .extend(tp.pos_li.into_iter().map(|i| i + offset));
    }
    self.txt_li.extend(tp.txt_li);
  }
}
