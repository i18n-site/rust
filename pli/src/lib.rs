use std::ops::{Index, IndexMut};

pub struct Pli<'a> {
  txt_li: &'a mut [String],
  pos_li: Vec<usize>,
}

impl<'a> Pli<'a> {
  pub fn len(&self) -> usize {
    self.pos_li.len()
  }

  pub fn is_empty(&self) -> bool {
    self.pos_li.is_empty()
  }

  pub fn new(txt_li: &'a mut [String], pos_li: Vec<usize>) -> Self {
    Pli { txt_li, pos_li }
  }

  pub fn iter(&'a self) -> impl Iterator<Item = &'a str> {
    self.pos_li.iter().map(move |&pos| &self.txt_li[pos][..])
  }

  pub fn iter_mut(&'a mut self) -> impl Iterator<Item = Pos<'a>> {
    let txt_li_ptr = self.txt_li as *mut [String];
    let pos_li = self.pos_li.clone();
    pos_li.into_iter().map(move |pos| Pos {
      txt_li: unsafe { &mut *txt_li_ptr },
      pos,
    })
  }
}

impl Index<usize> for Pli<'_> {
  type Output = String;

  fn index(&self, index: usize) -> &Self::Output {
    &self.txt_li[self.pos_li[index]]
  }
}

impl IndexMut<usize> for Pli<'_> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.txt_li[self.pos_li[index]]
  }
}

use std::ops::{Deref, DerefMut};

pub struct Pos<'a> {
  txt_li: &'a mut [String],
  pos: usize,
}

impl Deref for Pos<'_> {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.txt_li[self.pos]
  }
}

impl DerefMut for Pos<'_> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.txt_li[self.pos]
  }
}

impl Pos<'_> {
  pub fn as_str(&self) -> &str {
    &self.txt_li[self.pos]
  }
}

impl AsRef<str> for Pos<'_> {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}
