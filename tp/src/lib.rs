pub mod pos;
pub mod trim;
mod txtpos;
pub use txtpos::TxtPos;
pub fn whitespace_or_quote(c: char) -> bool {
  c.is_whitespace() || "\"'".contains(c)
}
