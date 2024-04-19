use xhash::HashLi;

use crate::TxtPos;

impl<'a> TxtPos<'a> {
  pub fn hash_li(&self) -> HashLi {
    HashLi::new(self.pos_li.iter().map(|i| self.txt_li[*i]))
  }
}
