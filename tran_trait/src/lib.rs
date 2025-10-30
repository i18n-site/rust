use aok::Void;
use txt_li::TxtLi;

pub trait Parse {
  fn parse<I: IntoIterator<Item = S>, S: Into<String>>(txt_li: &mut TxtLi, iter: I) -> Void;
}
